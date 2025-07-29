use calamine::{open_workbook, Reader, Xlsx, DataType, Result};

fn main() -> Result<()> {
    // Define the path to your Excel file
    let path = "ROTORMOD_V5_test_tool.xlsm";

    // --- Vectors to store column data ---
    let mut names: Vec<String> = Vec::new();
    let mut ages: Vec<f64> = Vec::new();
    let mut cities: Vec<String> = Vec::new();

    // Open the workbook
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // Get the worksheet by name. You could also use `workbook.worksheet_range_at(0)` for the first sheet.
    if let Some(Ok(range)) = workbook.worksheet_range("Rotor Export") {
        // Iterate over rows, skipping the first (header) row
        for row in range.rows().skip(1) {
            // Column A: Name (String)
            if let Some(cell) = row.get(0) {
                if let Some(name) = cell.get_string() {
                    names.push(name.to_string());
                }
            }

            // Column B: Age (Float/Int)
            if let Some(cell) = row.get(1) {
                if let Some(age) = cell.get_f64() {
                    ages.push(age);
                }
            }

            // Column C: City (String)
            if let Some(cell) = row.get(2) {
                // Using a match statement for demonstration
                match cell {
                    DataType::String(city) => cities.push(city.to_string()),
                    // You could add more match arms to handle other data types if necessary
                    _ => {}
                }
            }
        }
    } else {
        // Handle the case where the sheet is not found
        eprintln!("Error: Worksheet 'Sheet1' not found in {}", path);
    }

    // --- Print the resulting vectors to verify the data ---
    println!("## Data read successfully! ✅");

    println!("\nNames Vector:");
    println!("{:?}", names);

    println!("\nAges Vector:");
    println!("{:?}", ages);

    println!("\nCities Vector:");
    println!("{:?}", cities);

    // You can now use these vectors elsewhere in your program.
    // For example, finding the average age:
    let total_age: f64 = ages.iter().sum();
    let average_age = total_age / ages.len() as f64;
    println!("\nAverage Age: {:.2}", average_age);

    Ok(())
}