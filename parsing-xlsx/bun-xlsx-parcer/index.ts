import * as fs from "fs";
import * as xlsx from "xlsx";
import path from "path";
import { fileURLToPath } from "url";


interface ParsedRow {
  name: string;
  stock: number;
  price: number;
}

function parseExcelFile(
  filePath: string,
  logPath: string,
): { data: ParsedRow[]; errorCount: number } {
  const logStream = fs.createWriteStream(logPath, {
    flags: "w",
    encoding: "utf-8",
  });

  let workbook: xlsx.WorkBook;
  
  /* 
  workbook = xlsx.readFile(filePath, { cellDates: true }); 
  вызывает ошибку при build, поэтому лучше читать файл в буфер
  */

  try {
    const buffer = fs.readFileSync(filePath);
    workbook = xlsx.read(buffer, { type: "buffer", cellDates: true });
  } catch (e) {
    logStream.write(`Ошибка при чтении Excel: ${String(e)}\n`);
    throw e;
  }

  const sheetName = workbook.SheetNames[0];

  if (!sheetName) {
    throw new Error("Excel файл не содержит листов.");
  }

  const sheet = workbook.Sheets[sheetName];
  if (!sheet) {
    throw new Error(`Не найден лист с именем "${sheetName}"`);
  }

  const rows: Record<string, any>[] = xlsx.utils.sheet_to_json(sheet, {
    defval: null,
  });

  const supplierData: Record<string, ParsedRow> = {};
  let errorCount = 0;

  rows.forEach((row, index) => {
    const rowNum = index + 2;

    try {
      const name =
        typeof row["Название"] === "string" ? row["Название"].trim() : null;
      const stock = typeof row["Остаток"] === "number" ? row["Остаток"] : 0;
      const priceRaw =
        typeof row["Цена за 1 уп."] === "number" ? row["Цена за 1 уп."] : null;

      if (!name) {
        throw new Error("Пустое или некорректное название");
      }

      if (priceRaw === null) {
        throw new Error("Некорректная цена");
      }

      const price = Math.ceil(priceRaw);
      const existing = supplierData[name];

      if (!existing || price > existing.price) {
        supplierData[name] = { name, stock, price };
      }
    } catch (e) {
      errorCount++;
      const fallbackName =
        typeof row["Название"] === "string"
          ? row["Название"]
          : "(без названия)";
      logStream.write(
        `Ошибка в строке ${rowNum} '${fallbackName}': ${String(e)}\n`,
      );
    }
  });

  const dataList: ParsedRow[] = Object.values(supplierData);
  logStream.end();
  return { data: dataList, errorCount };
}

function main() {
  const inputFile = path.join(process.cwd(), "./supplier.xlsx");
  const outputLog = "./parse_errors_fast.log";
  const outputJson = "./parsed_data_fast.json";

  if (!fs.existsSync(inputFile)) {
    console.error("Ошибка: Файл 'supplier.xlsx' не найден.");
    return;
  }

  try {
    const { data, errorCount } = parseExcelFile(inputFile, outputLog);
    console.log(`Данные успешно спарсены: ${data.length} строк`);
    console.log(`Ошибок при обработке: ${errorCount}`);

    fs.writeFileSync(outputJson, JSON.stringify(data, null, 2), {
      encoding: "utf-8",
    });

    // Показать первые 50 символов из лога
    const logPreview = fs.readFileSync(outputLog, "utf-8").slice(0, 50);
    console.log("\nСообщения лога (первые 50 символов):");
    console.log(logPreview);
  } catch (e) {
    console.error(`Произошла ошибка: ${String(e)}`);
  }
}

main();
