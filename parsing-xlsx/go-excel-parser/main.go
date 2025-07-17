package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"

	"github.com/xuri/excelize/v2"
)

type SupplierEntry struct {
	Stock float64
	Price int
}

func parseExcelFile(path string, logFile *os.File) ([]map[string]any, error) {
	f, err := excelize.OpenFile(path)
	if err != nil {
		return nil, fmt.Errorf("ошибка при открытии файла: %w", err)
	}
	defer f.Close()

	sheetName := f.GetSheetName(0)
	if sheetName == "" {
		return nil, fmt.Errorf("активный лист не найден в Excel-файле")
	}

	rows, err := f.GetRows(sheetName)
	if err != nil {
		return nil, fmt.Errorf("ошибка при чтении строк: %w", err)
	}

	if len(rows) < 1 {
		return nil, fmt.Errorf("файл пустой")
	}

	headers := make(map[string]int)
	for idx, cell := range rows[0] {
		if cell != "" {
			headers[cell] = idx
		}
	}

	supplierData := make(map[string]SupplierEntry)

	for rowNum, row := range rows[1:] {
		var name string // нужен заранее, чтобы лог был информативным

		// Название
		nameIdx, ok := headers["Название"]
		if !ok || nameIdx >= len(row) {
			fmt.Fprintf(logFile, "Строка %d: колонка 'Название' не найдена или вне диапазона\n", rowNum+2)
			continue
		}
		name = row[nameIdx]
		if name == "" {
			fmt.Fprintf(logFile, "Строка %d: пустое название\n", rowNum+2)
			continue
		}

		// Остаток
		stockIdx, ok := headers["Остаток"]
		if !ok || stockIdx >= len(row) {
			fmt.Fprintf(logFile, "Строка %d: колонка 'Остаток' не найдена или вне диапазона для '%s'\n", rowNum+2, name)
			continue
		}
		stockStr := row[stockIdx]
		stock, err := strconv.ParseFloat(stockStr, 64)
		if err != nil {
			fmt.Fprintf(logFile, "Строка %d: не удалось распарсить 'Остаток'='%s' для '%s': %v\n", rowNum+2, stockStr, name, err)
			continue
		}

		// Цена
		priceIdx, ok := headers["Цена за 1 уп."]
		if !ok || priceIdx >= len(row) {
			fmt.Fprintf(logFile, "Строка %d: колонка 'Цена за 1 уп.' не найдена или вне диапазона для '%s'\n", rowNum+2, name)
			continue
		}
		priceStr := row[priceIdx]
		priceFloat, err := strconv.ParseFloat(priceStr, 64)
		if err != nil {
			fmt.Fprintf(logFile, "Строка %d: не удалось распарсить 'Цена за 1 уп.'='%s' для '%s': %v\n", rowNum+2, priceStr, name, err)
			continue
		}
		price := int(math.Ceil(priceFloat))

		entry, exists := supplierData[name]
		if !exists || price > entry.Price {
			supplierData[name] = SupplierEntry{
				Stock: stock,
				Price: price,
			}
		}
	}

	var result []map[string]any
	for name, data := range supplierData {
		result = append(result, map[string]any{
			"name":  name,
			"stock": data.Stock,
			"price": data.Price,
		})
	}

	return result, nil
}

func main() {
	logFile, err := os.Create("log-go.txt")
	if err != nil {
		log.Fatalf("не удалось создать лог-файл: %v", err)
	}
	defer logFile.Close()

	result, err := parseExcelFile("supplier.xlsx", logFile)
	if err != nil {
		log.Fatalf("Ошибка при парсинге: %v", err)
	}

	fmt.Println("Данные успешно спарсены (первые 5 строк):")
	for i, row := range result {
		if i >= 5 {
			break
		}
		fmt.Printf("%+v\n", row)
	}

	jsonFile, err := os.Create("result-go.json")
	if err != nil {
		log.Fatalf("не удалось создать JSON-файл: %v", err)
	}
	defer jsonFile.Close()

	encoder := json.NewEncoder(jsonFile)
	encoder.SetIndent("", "  ") // для читаемого формата
	if err := encoder.Encode(result); err != nil {
		log.Fatalf("не удалось сохранить JSON: %v", err)
	}

	fmt.Println("Результаты сохранены в result.json")

}
