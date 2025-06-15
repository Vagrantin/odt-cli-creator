use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use zip::{ZipWriter, write::FileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Calculate the first Wednesday of the following month
    let folder_name = get_first_wednesday_next_month();
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

fn get_first_wednesday_next_month() -> String {
    let today = Local::now().date_naive();
    let current_year = today.year();
    let current_month = today.month();
    
    // Calculate next month and year
    let (next_month, next_year) = if current_month == 12 {
        (1, current_year + 1)
    } else {
        (current_month + 1, current_year)
    };
    
    // Find the first Wednesday of next month
    let first_day = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
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
    format!("{:04}{:02}{:02}", 
            first_wednesday.year(), 
            first_wednesday.month(), 
            first_wednesday.day())
}

fn create_odt_document(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(path)?;
    let mut zip = ZipWriter::new(file);
    
    // Add mimetype (must be first and uncompressed)
    zip.start_file("mimetype", FileOptions::default().compression_method(zip::CompressionMethod::Stored))?;
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
        Command::new("open")
            .arg(&path_str.to_string())
            .spawn()
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
            .or_else(|_| {
                Command::new("xdg-open")
                    .arg(&path_str.to_string())
                    .spawn()
            })
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
