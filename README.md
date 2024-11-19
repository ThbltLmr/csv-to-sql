# WORK IN PROGRESS

## WHAT IS THIS
A CLI tool to parse a CSV file and generate a SQL file to update / insert rows

## USAGE
```
cargo run input_file.csv output_file.sql table_name
```

## NOTES
- Creates the output file if it does not exist
- Truncates the output file if it exists
- Ignores empty lines
- The first line of the `input_file.csv` should be headers
- The first column is used to identify rows (used in the `WHERE` clause)

## TODO
- Add flags to switch between update and insert
- Add insert mode
- Add option to have DB config and run output file automatically
- Add upsert mode, updating if id exists and inserting if it does not
