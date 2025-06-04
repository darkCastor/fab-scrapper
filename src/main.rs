use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// Base URL for fetching card set data from the API
const BASE_API_URL: &str = "https://cards.fabtcg.com/api/search/v1/cards/?set_code=";

// Input file containing set codes
const SET_CODES_FILENAME: &str = "sets_codes.txt";

/// Reads set codes from the specified file, one code per line.
///
/// # Arguments
/// * `filename` - The path to the file containing set codes.
///
/// # Returns
/// A `Result` containing a vector of set codes if successful, or an error.
fn read_set_codes(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Check if the file exists
    if !Path::new(filename).exists() {
        return Err(format!("Error: Input file '{}' not found. Please create it in the project root.", filename).into());
    }

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut codes = Vec::new();

    for line in reader.lines() {
        let line_content = line?.trim().to_string();
        if !line_content.is_empty() {
            codes.push(line_content);
        }
    }
    Ok(codes)
}

/// Fetches JSON data for a given set code from the cards.fabtcg.com API.
///
/// # Arguments
/// * `set_code` - The set code (e.g., "WTR").
///
/// # Returns
/// A `Result` containing the JSON response as a string if successful, or an error.
fn fetch_set_json_data(set_code: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_API_URL, set_code.trim());
    println!("Fetching JSON from URL: {}", url);

    // Make a blocking GET request
    let client = reqwest::blocking::Client::builder()
        .user_agent("fab-card-collector-rust-script/1.0") // Good practice to set a User-Agent
        .build()?;
        
    let response = client.get(&url).send()?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!(
            "Request to {} failed with status: {}",
            url,
            response.status()
        )
        .into());
    }

    // Read the response body as text (JSON string)
    let body = response.text()?;
    Ok(body)
}

/// Saves the provided data string to a file.
///
/// # Arguments
/// * `filename` - The name of the file to save the data to.
/// * `data` - The string data to write to the file.
///
/// # Returns
/// A `Result` indicating success or an error.
fn save_data_to_file(filename: &str, data: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Main function to drive the script.
fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Flesh and Blood Card API Data Collector\nReading set codes from: {}",
        SET_CODES_FILENAME
    );

    // Read set codes from the file
    let set_codes = match read_set_codes(SET_CODES_FILENAME) {
        Ok(codes) => codes,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("Please ensure '{}' exists in the same directory as the executable or in the project root if using 'cargo run'.", SET_CODES_FILENAME);
            eprintln!("The file should contain one set code per line (e.g., WTR, ARC).");
            return Err(e); // Propagate the error to stop execution
        }
    };

    if set_codes.is_empty() {
        println!("No set codes found in {}. Exiting.", SET_CODES_FILENAME);
        return Ok(());
    }

    println!("Found {} set codes to process.", set_codes.len());

    // Create a directory for output files if it doesn't exist
    let output_dir = "set_data_json_txt"; // Changed directory name
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
        println!("Created output directory: {}", output_dir);
    }


    // Process each set code
    for set_code in set_codes {
        println!("\nProcessing set: {}", set_code);
        match fetch_set_json_data(&set_code) {
            Ok(json_content) => {
                // Construct the output filename, placing it in the output directory
                // Save as .txt as requested
                let output_filename = format!("{}/{}_cards.txt", output_dir, set_code.trim()); 
                println!("Saving JSON data to: {}", output_filename);

                if let Err(e) = save_data_to_file(&output_filename, &json_content) {
                    eprintln!(
                        "Error saving file {}: {}. Skipping this set.",
                        output_filename, e
                    );
                } else {
                    println!("Successfully saved {}", output_filename);
                }
            }
            Err(e) => {
                eprintln!(
                    "Error fetching JSON data for set {}: {}. Skipping this set.",
                    set_code, e
                );
            }
        }

        // Optional: Add a small delay to be polite to the server.
        // This is even more important when hitting an API directly.
        std::thread::sleep(std::time::Duration::from_millis(500)); // 500ms delay
    }

    println!("\nFinished processing all set codes. JSON data (saved as .txt) files are in the '{}' directory.", output_dir);
    Ok(())
}
