use calamine::{Reader, open_workbook_auto, RangeDeserializerBuilder};
use std::path::PathBuf;
use serde::Deserialize;

// Define a struct to hold our row data.
// The #[serde(rename = "...")] attribute maps Excel column headers to struct fields.
// Option<f64> is used for 'score' to handle potentially empty cells gracefully.
#[derive(Debug, Deserialize)]
struct station {
    #[serde(rename = "Station")]
    element: int,
    #[serde(rename = "Label")]
    label: string,
    #[serde(rename = "Length (in)")]
    length: f64,
    #[serde(rename = "OD K")]
    outer_diameter_stiffness: f64,
    #[serde(rename = "OD M")]
    outer_diameter_mass: f64,
    #[serde(rename = "ID K")]
    inner_diameter_stiffness: f64,
    #[serde(rename = "ID M")]
    inner_diameter_mass: f64,
    #[serde(rename = "E")]
    youngs_modulus: f64,
    #[serde(rename = "v")]
    poisson_ration: f64,
    #[serde(rename = "density")]
    density: f64,
    #[serde(rename = "Added W")]
    added_mass: f64,
    #[serde(rename = "lp")]
    lp: f64,
    #[serde(rename = "It")]
    it: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PathBuf::from("ROTORMOD_V5_test_tool.xlsx");
    let sheet_name = "Rotor Export"; // Specify the sheet name you want to read

    // Check if the Excel file exists
    if !file_path.exists() {
        eprintln!("Error: Excel file not found at {:?}", file_path);
        eprintln!("Please ensure '{}' in the project root directory.", file_path);
        eprintln!("Ensure your data is in the sheet named '{}'.", sheet_name);
        return Ok(());
    }

    println!("--- Reading data from '{}' in sheet '{}' ---", file_path, sheet_name);

    // Open the workbook. `open_workbook_auto` automatically detects the file type.
    let mut workbook = open_workbook_auto(&file_path)?;

    // Get the range (data area) of the specified sheet.
    // The '?' operator will return an error if the sheet is not found or other issues occur.
    if let Some(range) = workbook.worksheet_range(sheet_name)? {
        // Create a deserializer builder for the range.
        // `has_headers(true)` is crucial if your first row contains column names.
        let mut reader = RangeDeserializerBuilder::new()
            .has_headers(true)
            .from_range(&range)?;

        // Iterate over the deserialized rows.
        // Each `result` is a Result<Person, calamine::Error>.
        for result in reader {
            match result {
                Ok(person) => {
                    // Successfully deserialized a row into a Person struct
                    println!("Successfully read: {:?}", person);
                }
                Err(e) => {
                    // Handle errors during deserialization for a specific row
                    eprintln!("Error deserializing row: {}", e);
                }
            }
        }
    } else {
        // Handle the case where the specified sheet was not found
        eprintln!("Error: Sheet '{}' not found in workbook.", sheet_name);
    }

    Ok(())
}