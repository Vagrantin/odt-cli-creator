# ODT Creator CLI

A simple Rust command-line tool that creates OpenDocument Text (ODT) files and organizes them by date.

## Features

- Creates a folder named after the first Wednesday of a specified month (YYYYMMDD format)
- Generates a properly formatted ODT document with user-specified filename
- Automatically opens the created document with the default application
- Flexible month selection with command-line parameters
- Smart year handling for past months

## Dependencies

This tool uses the following Rust crates:
- `chrono` - Date and time handling
- `zip` - Creating ODT files (which are ZIP archives)
- Standard library modules for file I/O and process management

## Usage

### Basic Usage (Default Behavior)

Run without parameters to create a folder for next month's first Wednesday:

```bash
cargo run
```

### Advanced Usage with Month Parameter

Specify a target month using the `--month` or `-m` parameter:

```bash
# Create folder for September's first Wednesday
cargo run -- -m 9
cargo run -- --month 9

# Create folder for December's first Wednesday
cargo run -- --month 12
```

### Getting Help

```bash
cargo run -- --help
```

## Command Line Options

- `-m, --month <MONTH>`: Specify the month (1-12) for which to create the folder
- `-h, --help`: Show help message and usage examples

## Examples

### Default behavior (next month):
```
$ cargo run
Creating folder: 20250702
Enter the filename for the ODT document (without extension): meeting-notes
Created ODT document: 20250702/meeting-notes.odt
Opening document with default application...
```

### Specifying September:
```
$ cargo run -- -m 9
Creating folder: 20250903
Enter the filename for the ODT document (without extension): quarterly-report
Created ODT document: 20250903/quarterly-report.odt
Opening document with default application...
```

### Getting help:
```
$ cargo run -- --help
Usage: target/debug/odt_creator [OPTIONS]

Options:
  -m, --month <MONTH>    Specify the month (1-12) for which to create the folder
                         If not specified, uses the following month
  -h, --help             Show this help message

Examples:
  target/debug/odt_creator                     # Creates folder for next month's first Wednesday
  target/debug/odt_creator -m 9                # Creates folder for September's first Wednesday
  target/debug/odt_creator --month 12          # Creates folder for December's first Wednesday
```

## Smart Year Handling

The tool intelligently handles year selection:

- **No month specified**: Uses the following month
- **Future month**: Uses the current year (e.g., specifying month 9 in July 2025 creates folder for September 2025)
- **Past month**: Uses the next year (e.g., specifying month 3 in July 2025 creates folder for March 2026)

## Requirements

- Rust toolchain
- For automatic document opening:
  - **Windows**: Uses system default application
  - **macOS**: Uses the `open` command
  - **Linux**: LibreOffice, OpenOffice, or xdg-open

## How it Works

The tool calculates the first Wednesday of the target month (either next month by default or a specified month) and creates a folder with that date. It then generates a valid ODT file by creating a ZIP archive containing the necessary XML files that conform to the OpenDocument format specification.

## Error Handling

- Validates that the filename is not empty
- Validates month parameter is between 1 and 12
- Provides clear error messages for invalid inputs
- Provides fallback options for opening documents on different platforms
- Gracefully handles cases where automatic document opening fails

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
