// src/main.rs
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Result, anyhow};
use calamine::{DataType, Reader, open_workbook_auto};
use serde_json::json;

#[derive(Debug)]
struct SupplierEntry {
    name: String,
    stock: f64,
    price: i64,
}

fn parse_excel_file<P: AsRef<Path>>(
    file_path: P,
    mut log: BufWriter<File>,
) -> Result<(Vec<HashMap<String, String>>, usize, usize)> {
    let mut workbook = open_workbook_auto(&file_path)?;
    let range = workbook
        .worksheet_range_at(0)
        .ok_or_else(|| anyhow!("Активный лист не найден в Excel-файле"))??;

    let mut rows = range.rows();
    let headers_row = rows.next().ok_or_else(|| anyhow!("Пустой Excel-файл"))?;

    let headers: HashMap<String, usize> = headers_row
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| {
            if cell.is_string() {
                cell.get_string().map(|s| (s.to_string(), i))
            } else {
                None
            }
        })
        .collect();

    let mut supplier_data: HashMap<String, SupplierEntry> = HashMap::new();
    let mut success_count = 0usize;
    let mut error_count = 0usize;

    for row in rows {
        let mut name_opt: Option<String> = None;

        let result = (|| {
            let name_cell = row.get(*headers.get("Название")?)?;
            if !name_cell.is_string() {
                return None;
            }
            let name = name_cell.get_string()?.to_string();
            name_opt = Some(name.clone());

            let stock_cell = row.get(*headers.get("Остаток")?)?;
            let stock = if stock_cell.is_float() {
                stock_cell.get_float()?
            } else if stock_cell.is_int() {
                stock_cell.get_int()? as f64
            } else {
                return None;
            };

            let price_cell = row.get(*headers.get("Цена за 1 уп.")?)?;
            let price = if price_cell.is_float() {
                price_cell.get_float()?.ceil() as i64
            } else if price_cell.is_int() {
                price_cell.get_int()?
            } else {
                return None;
            };

            Some(SupplierEntry { name, stock, price })
        })();

        match result {
            Some(entry) => {
                supplier_data.insert(entry.name.clone(), entry);
                success_count += 1;
            }
            None => {
                error_count += 1;
                let _ = writeln!(
                    log,
                    "Ошибка в строке с товаром{}",
                    name_opt
                        .as_ref()
                        .map(|n| format!(" '{}'", n))
                        .unwrap_or_default()
                );
            }
        }
    }

    let result: Vec<HashMap<String, String>> = supplier_data
        .into_iter()
        .map(|(name, supplier)| {
            let mut map = HashMap::new();
            map.insert("name".into(), name);
            map.insert("stock".into(), supplier.stock.to_string());
            map.insert("price".into(), supplier.price.to_string());
            map
        })
        .collect();

    Ok((result, success_count, error_count))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <excel_file> <output_json>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let log_file = File::create("log.txt")?;
    let log_writer = BufWriter::new(log_file);

    let (result, success_count, error_count) = parse_excel_file(input_path, log_writer)?;

    let json = json!(result);
    std::fs::write(output_path, serde_json::to_string_pretty(&json)?)?;

    println!("✅ Обработано строк: {}", success_count);
    println!("⚠️ Строк с ошибками: {}", error_count);

    Ok(())
}
