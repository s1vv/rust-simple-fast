| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./excel-parser-rs supplier.xlsx out-rs.json` | 437.2 ± 6.6 | 430.4 | 450.0 | 1.00 |
| `uv run parsing_fast_openpyxl.py` | 5023.9 ± 68.6 | 4929.1 | 5164.4 | 11.49 ± 0.23 |
| `uv run parsing_slow_pd.py` | 7252.9 ± 82.0 | 7151.3 | 7379.7 | 16.59 ± 0.31 |
