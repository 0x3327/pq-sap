use std::{error::Error, fs::{self, File}, io::Write};

pub const METADATA_PATH: &str = "src/on_chain/data/meta_data.json";

pub fn read_meta_data() -> u64{
    let meta_data = fs::read_to_string(METADATA_PATH).unwrap_or_default();
    let meta_data = serde_json::from_str(&meta_data).unwrap_or_else(|_| serde_json::json!({})); 
    let latest_block = meta_data.get("latest_block")
    .and_then(|value| value.as_u64())
    .unwrap_or(0); 

    latest_block
}

pub fn write_meta_data(latest_block: u64) -> Result<(), Box<dyn Error>>{
    let meta_data = fs::read_to_string(METADATA_PATH).unwrap_or_default();
    let mut meta_data = serde_json::from_str(&meta_data).unwrap_or_else(|_| serde_json::json!({})); 

    meta_data["latest_block"] = serde_json::json!(latest_block);
    let json_string = serde_json::to_string_pretty(&meta_data)?;
    let mut file = File::create(METADATA_PATH)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn is_valid_hex(hex_str: &String) -> bool{
    let trimmed = if hex_str.starts_with("0x") { &hex_str[2..] } else { &hex_str[..] };

    trimmed.chars().all(|c| c.is_digit(16))
}