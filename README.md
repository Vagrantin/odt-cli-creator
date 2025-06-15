# ODT Creator CLI

A simple Rust command-line tool that creates OpenDocument Text (ODT) files and organizes them by date.

## Features

- Creates a folder named after the first Wednesday of the next month (YYYYMMDD format)
- Generates a properly formatted ODT document with user-specified filename
- Automatically opens the created document with the default application

## Dependencies

This tool uses the following Rust crates:
- `chrono` - Date and time handling
- `zip` - Creating ODT files (which are ZIP archives)
- Standard library modules for file I/O and process management

## Usage

1. Run the program:
   ```bash
   cargo run
   ```

2. Enter a filename when prompted (without the .odt extension)

3. The tool will:
   - Create a folder with the date of the first Wednesday of next month
   - Generate an ODT document with your specified filename
   - Attempt to open the document automatically

## Example

```
$ cargo run
Creating folder: 20250702
Enter the filename for the ODT document (without extension): meeting-notes
Created ODT document: 20250702/meeting-notes.odt
Opening document with default application...
```

## Requirements

- Rust toolchain
- For automatic document opening:
  - **Linux**: LibreOffice, OpenOffice, or xdg-open

## How it Works

The tool calculates the first Wednesday of the following month and creates a folder with that date. It then generates a valid ODT file by creating a ZIP archive containing the necessary XML files that conform to the OpenDocument format specification.

## Error Handling

- Validates that the filename is not empty
- Provides fallback options for opening documents on different platforms
- Gracefully handles cases where automatic document opening fails

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
