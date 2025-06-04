use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashMap;
use chrono::{DateTime, Local};

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
    let script_launch_time: DateTime<Local> = Local::now();
    
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

    // Create directories for output files if they don't exist
    let base_output_dir = "script_generated_card_data";
    let txt_output_dir = format!("{}/txt", base_output_dir);
    let json_output_dir = format!("{}/json", base_output_dir);
    
    if !Path::new(base_output_dir).exists() {
        fs::create_dir(base_output_dir)?;
        println!("Created base output directory: {}", base_output_dir);
    }
    if !Path::new(&txt_output_dir).exists() {
        fs::create_dir(&txt_output_dir)?;
        println!("Created txt output directory: {}", txt_output_dir);
    }
    if !Path::new(&json_output_dir).exists() {
        fs::create_dir(&json_output_dir)?;
        println!("Created json output directory: {}", json_output_dir);
    }

    // HashMap to store all set data for the combined file
    let mut all_sets_data: HashMap<String, String> = HashMap::new();

    // Process each set code
    for set_code in &set_codes {
        println!("\nProcessing set: {}", set_code);
        match fetch_set_json_data(set_code) {
            Ok(json_content) => {
                // Construct the output filenames for both txt and json versions
                let txt_filename = format!("{}/{}_cards.txt", txt_output_dir, set_code.trim()); 
                let json_filename = format!("{}/{}_cards.json", json_output_dir, set_code.trim()); 
                
                println!("Saving data to: {} and {}", txt_filename, json_filename);

                // Save txt version
                let mut txt_success = false;
                if let Err(e) = save_data_to_file(&txt_filename, &json_content) {
                    eprintln!("Error saving txt file {}: {}", txt_filename, e);
                } else {
                    println!("Successfully saved {}", txt_filename);
                    txt_success = true;
                }

                // Save json version
                let mut json_success = false;
                if let Err(e) = save_data_to_file(&json_filename, &json_content) {
                    eprintln!("Error saving json file {}: {}", json_filename, e);
                } else {
                    println!("Successfully saved {}", json_filename);
                    json_success = true;
                }

                // Store the data for the combined file if at least one save was successful
                if txt_success || json_success {
                    all_sets_data.insert(set_code.trim().to_string(), json_content);
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

    // Create the combined files with all sets data
    if !all_sets_data.is_empty() {
        println!("\nCreating combined files with all sets data...");
        let combined_txt_filename = format!("{}/all_sets_combined.txt", txt_output_dir);
        let combined_json_filename = format!("{}/all_sets_combined.json", json_output_dir);
        
        // Create a JSON object with all sets
        let mut combined_json = String::from("{\n");
        let mut first = true;
        for (set_code, json_data) in &all_sets_data {
            if !first {
                combined_json.push_str(",\n");
            }
            combined_json.push_str(&format!("  \"{}\": {}", set_code, json_data));
            first = false;
        }
        combined_json.push_str("\n}");
        
        // Save combined txt version
        if let Err(e) = save_data_to_file(&combined_txt_filename, &combined_json) {
            eprintln!("Error saving combined txt file {}: {}", combined_txt_filename, e);
        } else {
            println!("Successfully saved combined txt file: {}", combined_txt_filename);
        }

        // Save combined json version
        if let Err(e) = save_data_to_file(&combined_json_filename, &combined_json) {
            eprintln!("Error saving combined json file {}: {}", combined_json_filename, e);
        } else {
            println!("Successfully saved combined json file: {}", combined_json_filename);
        }
    }

    // Create metadata file with script info
    let unknown_set = String::from("UNKNOWN");
    let latest_set = set_codes.last().unwrap_or(&unknown_set);
    let metadata_filename = format!("{}/script_metadata.txt", base_output_dir);
    let metadata_content = format!(
        "FAB Card Scrapper - Script Execution Metadata\n\
        =============================================\n\
        Script Launch Time: {}\n\
        Latest Set Processed: {}\n\
        Total Sets Processed: {}\n\
        Sets List: {}\n\
        Output Structure:\n\
        - TXT files: {}/\n\
        - JSON files: {}/\n",
        script_launch_time.format("%Y-%m-%d %H:%M:%S %Z"),
        latest_set,
        all_sets_data.len(),
        set_codes.join(", "),
        txt_output_dir,
        json_output_dir
    );
    
    if let Err(e) = save_data_to_file(&metadata_filename, &metadata_content) {
        eprintln!("Warning: Could not save metadata file {}: {}", metadata_filename, e);
    } else {
        println!("Created metadata file: {}", metadata_filename);
    }

    println!("\nFinished processing all set codes. Files are organized in '{}' directory:", base_output_dir);
    println!("  - TXT files: {}/", txt_output_dir);
    println!("  - JSON files: {}/", json_output_dir);
    println!("  - Metadata: {}", metadata_filename);
    Ok(())
}
