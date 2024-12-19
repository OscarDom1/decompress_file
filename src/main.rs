use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use zip::read::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the ZIP file
    let zip_file_path = "main.pdf.zip";

    // Open the ZIP file
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    // Iterate through all the files in the ZIP archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        // Ensure safe file path
        let outpath = file.sanitized_name(); 

        println!("Extracting: {}", outpath.display());

        // Check if the entry is a directory or a file
        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            // Ensure the parent directory exists
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            // Write the file content to the output path
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("Extraction complete. Kindly check your file");
    Ok(())
}
