| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./excel-parser-rs supplier.xlsx out-rs.json` | 439.3 ± 7.2 | 430.5 | 451.9 | 1.00 |
| `./go-xlsx-parser` | 2011.5 ± 51.9 | 1935.3 | 2077.2 | 4.58 ± 0.14 |
| `zsh -c "bun run ./index.js"` | 2070.5 ± 35.5 | 2013.0 | 2144.2 | 4.71 ± 0.11 |
| `python3 ./parsing_fast_openpyxl.py` | 5034.9 ± 46.1 | 4984.2 | 5135.8 | 11.46 ± 0.22 |
