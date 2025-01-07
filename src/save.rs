use serde_json::json;
use std::collections::HashMap;
use std::fs;

pub fn save_to_file(
    file_name: &str,
    data: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = json!(data);
    let mut file_path = format!("{}", file_name);
    // Ensure the filename has a .json extension
    if !file_path.ends_with(".json") {
        file_path.push_str(".json");
    }

    fs::write(&file_path, serde_json::to_string_pretty(&json_data)?)?;
    println!("Data successfully saved to {}", file_path);

    Ok(())
}
