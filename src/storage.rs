use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::models::Patient;
use serde_json;

pub fn load_patients(path: &Path) -> Result<Vec<Patient>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path).map_err(|e| format!("Could not open data file: {}", e))?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).map_err(|e| format!("Could not read data file: {}", e))?;
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(&contents).map_err(|e| format!("Invalid JSON in data file: {}", e))
}

pub fn save_patients(path: &Path, patients: &[Patient]) -> Result<(), String> {
    let file = File::create(path).map_err(|e| format!("Could not create data file: {}", e))?;
    let mut writer = BufWriter::new(file);
    let json = serde_json::to_string_pretty(patients).map_err(|e| format!("Serialization error: {}", e))?;
    writer.write_all(json.as_bytes()).map_err(|e| format!("Could not write data file: {}", e))?;
    writer.flush().map_err(|e| format!("Could not flush data file: {}", e))?;
    Ok(())
}
