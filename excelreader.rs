//reads ROTORMOD excel file

use calamine::{open_workbook_auto, Reader, Sheets};
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Construct the path to the Excel file
    // Assuming the Excel file is in the same directory as your Cargo.toml
    let mut current_dir = env::current_dir()?;
    current_dir.push("ROTORMOD_V5_NNL_Phase_1_Test_Rig_2.0.xlsx"); // Original Excel file name

    let path: PathBuf = current_dir;
    let sheet_name = "Rotor Export";

    println!("Attempting to open Excel file: {:?}", path);
    println!("Looking for sheet: \"{}\"", sheet_name);

    // Open the workbook automatically detecting the file type
    let mut excel_workbook: Sheets = open_workbook_auto(&path)?;

    // Get the specific sheet by name
    if let Some(Ok(range)) = excel_workbook.worksheet_range(sheet_name) {
        println!("\nSuccessfully found and read \"{}\" sheet.", sheet_name);
        println!("Sheet dimensions: {:?}", range.get_size());
        println!("Number of rows: {}", range.get_size().0);
        println!("Number of columns: {}", range.get_size().1);
        println!("\n--- Content of \"Rotor Export\" sheet ---");

        // Iterate over rows and print cell values
        for row in range.rows() {
            for (i, cell) in row.iter().enumerate() {
                // Print cell value, followed by a tab.
                // You might want to format this based on your specific needs.
                print!("{}", cell);
                if i < row.len() - 1 {
                    print!("\t"); // Add a tab between cells
                }
            }
            println!(); // Newline after each row
        }
    } else {
        eprintln!("Error: Sheet \"{}\" not found in the workbook.", sheet_name);
        // You can list available sheets for debugging:
        // println!("Available sheets: {:?}", excel_workbook.sheet_names());
    }

    Ok(())
}
