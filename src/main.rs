use chrono::{Datelike, Local, NaiveDate, Weekday};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use zip::{write::FileOptions, ZipWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let target_month = parse_month_argument(&args)?;

    // Calculate the first Wednesday of the target month
    let folder_name = get_first_wednesday_for_month(target_month);
    println!("Creating folder: {}", folder_name);

    // Create the folder
    fs::create_dir_all(&folder_name)?;

    // Get user input for filename
    print!("Enter the filename for the ODT document (without extension): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let filename = input.trim();

    if filename.is_empty() {
        eprintln!("Error: Filename cannot be empty");
        return Ok(());
    }

    // Create the full path for the ODT file
    let odt_filename = format!("{}.odt", filename);
    let odt_path = Path::new(&folder_name).join(&odt_filename);

    // Create the ODT document
    create_odt_document(&odt_path)?;
    println!("Created ODT document: {}", odt_path.display());

    // Open the document with LibreOffice/OpenOffice
    open_document(&odt_path)?;

    Ok(())
}

fn parse_month_argument(args: &[String]) -> Result<Option<u32>, Box<dyn std::error::Error>> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--month" | "-m" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --month/-m requires a value (1-12)");
                    print_usage();
                    std::process::exit(1);
                }
                let month_str = &args[i + 1];
                match month_str.parse::<u32>() {
                    Ok(month) if month >= 1 && month <= 12 => return Ok(Some(month)),
                    _ => {
                        eprintln!(
                            "Error: Month must be a number between 1 and 12, got: {}",
                            month_str
                        );
                        print_usage();
                        std::process::exit(1);
                    }
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            arg if arg.starts_with('-') => {
                eprintln!("Error: Unknown option: {}", arg);
                print_usage();
                std::process::exit(1);
            }
            _ => {
                // Skip non-option arguments
            }
        }
        i += 1;
    }
    Ok(None)
}

fn print_usage() {
    println!(
        "Usage: {} [OPTIONS]",
        env::args()
            .next()
            .unwrap_or_else(|| "odt_creator".to_string())
    );
    println!();
    println!("Options:");
    println!("  -m, --month <MONTH>    Specify the month (1-12) for which to create the folder");
    println!("                         If not specified, uses the following month");
    println!("  -h, --help             Show this help message");
    println!();
    println!("Examples:");
    println!(
        "  {}                     # Creates folder for next month's first Wednesday",
        env::args()
            .next()
            .unwrap_or_else(|| "odt_creator".to_string())
    );
    println!(
        "  {} -m 9                # Creates folder for September's first Wednesday",
        env::args()
            .next()
            .unwrap_or_else(|| "odt_creator".to_string())
    );
    println!(
        "  {} --month 12          # Creates folder for December's first Wednesday",
        env::args()
            .next()
            .unwrap_or_else(|| "odt_creator".to_string())
    );
}

fn get_first_wednesday_for_month(target_month: Option<u32>) -> String {
    let today = Local::now().date_naive();
    let current_year = today.year();

    let (month, year) = match target_month {
        Some(m) => {
            // Use specified month with current year
            // If the specified month has already passed this year, use next year
            if m < today.month() {
                (m, current_year + 1)
            } else {
                (m, current_year)
            }
        }
        None => {
            // Original behavior: next month
            let current_month = today.month();
            if current_month == 12 {
                (1, current_year + 1)
            } else {
                (current_month + 1, current_year)
            }
        }
    };

    // Find the first Wednesday of the target month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first_day.weekday();

    let days_until_wednesday = match first_weekday {
        Weekday::Wed => 0,
        Weekday::Thu => 6,
        Weekday::Fri => 5,
        Weekday::Sat => 4,
        Weekday::Sun => 3,
        Weekday::Mon => 2,
        Weekday::Tue => 1,
    };

    let first_wednesday = first_day + chrono::Duration::days(days_until_wednesday);

    // Format as YYYYMMDD
    format!(
        "{:04}{:02}{:02}",
        first_wednesday.year(),
        first_wednesday.month(),
        first_wednesday.day()
    )
}

fn create_odt_document(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(path)?;
    let mut zip = ZipWriter::new(file);

    // Add mimetype (must be first and uncompressed)
    zip.start_file(
        "mimetype",
        FileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )?;
    zip.write_all(b"application/vnd.oasis.opendocument.text")?;

    // Add META-INF/manifest.xml
    zip.start_file("META-INF/manifest.xml", FileOptions::default())?;
    let manifest = r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0">
    <manifest:file-entry manifest:full-path="/" manifest:media-type="application/vnd.oasis.opendocument.text"/>
    <manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml"/>
    <manifest:file-entry manifest:full-path="styles.xml" manifest:media-type="text/xml"/>
    <manifest:file-entry manifest:full-path="meta.xml" manifest:media-type="text/xml"/>
</manifest:manifest>"#;
    zip.write_all(manifest.as_bytes())?;

    // Add content.xml
    zip.start_file("content.xml", FileOptions::default())?;
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" 
                        xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
                        xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
                        xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0">
    <office:body>
        <office:text>
            <text:p text:style-name="Standard">This is a new ODT document created by Rust CLI tool.</text:p>
        </office:text>
    </office:body>
</office:document-content>"#;
    zip.write_all(content.as_bytes())?;

    // Add styles.xml
    zip.start_file("styles.xml", FileOptions::default())?;
    let styles = r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-styles xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
                       xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
                       xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
                       xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0">
    <office:styles>
        <style:default-style style:family="paragraph">
            <style:paragraph-properties fo:hyphenation-ladder-count="no-limit"/>
            <style:text-properties fo:language="en" fo:country="US"/>
        </style:default-style>
        <style:style style:name="Standard" style:family="paragraph" style:class="text"/>
    </office:styles>
</office:document-styles>"#;
    zip.write_all(styles.as_bytes())?;

    // Add meta.xml
    zip.start_file("meta.xml", FileOptions::default())?;
    let meta = r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-meta xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
                     xmlns:meta="urn:oasis:names:tc:opendocument:xmlns:meta:1.0">
    <office:meta>
        <meta:generator>Rust CLI ODT Creator</meta:generator>
        <meta:creation-date>2025-06-13T00:00:00</meta:creation-date>
    </office:meta>
</office:document-meta>"#;
    zip.write_all(meta.as_bytes())?;

    zip.finish()?;
    Ok(())
}

fn open_document(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_string_lossy();

    // Try different commands based on the operating system
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", "", &path_str])
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(&path_str.to_string()).spawn()
    } else {
        // Linux and other Unix-like systems
        // Try LibreOffice first, then OpenOffice, then xdg-open
        Command::new("libreoffice")
            .arg(&path_str.to_string())
            .spawn()
            .or_else(|_| {
                Command::new("openoffice")
                    .arg(&path_str.to_string())
                    .spawn()
            })
            .or_else(|_| Command::new("xdg-open").arg(&path_str.to_string()).spawn())
    };

    match result {
        Ok(_) => {
            println!("Opening document with default application...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Warning: Could not open document automatically: {}", e);
            eprintln!("Please open the file manually: {}", path_str);
            Ok(())
        }
    }
}
