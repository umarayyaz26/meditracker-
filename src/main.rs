mod io_utils;
mod models;
mod storage;

use clap::Parser;
use std::io::Write;
use std::path::PathBuf;

use models::{Medication, Patient};
use storage::{load_patients, save_patients};

#[derive(Parser)]
#[command(name = "meditracker")]
#[command(author, version, about = "CLI tool for managing patient and medication records")]
struct Cli {
    /// Path to the JSON data file (default: ~/.meditracker/data.json)
    #[arg(short, long, value_name = "FILE")]
    data_path: Option<PathBuf>,
}

fn display_menu() {
    println!("\n╔══════════════════════════════════════╗");
    println!("║         MediTracker v1.0.2           ║");
    println!("╚══════════════════════════════════════╝");
    println!("  1. Add Patient");
    println!("  2. View Patients");
    println!("  3. Add Medication");
    println!("  4. View Medications");
    println!("  5. Remove Medication");
    println!("  6. Update Medication");
    println!("  7. Remove Patient");
    println!("  8. Export to CSV");
    println!("  9. Exit");
    print!("\n  Enter your choice: ");
    std::io::stdout().flush().unwrap();
}

fn add_patient(patients: &mut Vec<Patient>) {
    let id = io_utils::read_line("Patient ID:");
    let name = io_utils::read_line("Patient name:");
    let age = io_utils::read_u32("Patient age:");
    let gender = io_utils::read_line("Gender:");
    let disease = io_utils::read_line("Disease/condition:");

    if patients.iter().any(|p| p.id == id) {
        println!("Error: A patient with ID '{}' already exists.", id);
        return;
    }

    patients.push(Patient {
        id,
        name,
        age,
        gender,
        disease,
        medications: Vec::new(),
    });
    println!("✓ Patient added successfully.");
}

fn view_patients(patients: &[Patient]) {
    if patients.is_empty() {
        println!("\n  No patients found.");
        return;
    }
    println!("\n  Patients ({}):", patients.len());
    for p in patients {
        println!(
            "    • {} | {} | {} | {} | {} med(s)",
            p.id, p.name, p.gender, p.disease, p.medications.len()
        );
    }
}

fn add_medication(patients: &mut Vec<Patient>) {
    let id = io_utils::read_line("Patient ID:");
    if let Some(p) = patients.iter_mut().find(|x| x.id == id) {
        let name = io_utils::read_line("Medication name:");
        let schedule = io_utils::read_line("Schedule (e.g. 8 AM, 2 PM, 8 PM):");
        p.medications.push(Medication { name, schedule });
        println!("✓ Medication added for {}.", p.name);
    } else {
        println!("Patient not found.");
    }
}

fn view_medications(patients: &[Patient]) {
    for p in patients {
        println!("\n  {} (ID: {})", p.name, p.id);
        if p.medications.is_empty() {
            println!("    No medications.");
        } else {
            for m in &p.medications {
                println!("    • {} — {}", m.name, m.schedule);
            }
        }
    }
}

fn remove_medication(patients: &mut Vec<Patient>) {
    let id = io_utils::read_line("Patient ID:");
    if let Some(p) = patients.iter_mut().find(|x| x.id == id) {
        let name = io_utils::read_line("Medication name to remove:");
        if let Some(pos) = p.medications.iter().position(|m| m.name == name) {
            p.medications.remove(pos);
            println!("✓ Medication removed from {}.", p.name);
        } else {
            println!("Medication not found.");
        }
    } else {
        println!("Patient not found.");
    }
}

fn update_medication(patients: &mut Vec<Patient>) {
    let id = io_utils::read_line("Patient ID:");
    if let Some(p) = patients.iter_mut().find(|x| x.id == id) {
        let name = io_utils::read_line("Medication name to update:");
        if let Some(m) = p.medications.iter_mut().find(|x| x.name == name) {
            m.schedule = io_utils::read_line("New schedule:");
            println!("✓ Medication updated.");
        } else {
            println!("Medication not found.");
        }
    } else {
        println!("Patient not found.");
    }
}

fn remove_patient(patients: &mut Vec<Patient>) {
    let id = io_utils::read_line("Patient ID to remove:");
    if let Some(pos) = patients.iter().position(|p| p.id == id) {
        patients.remove(pos);
        println!("✓ Patient removed.");
    } else {
        println!("Patient not found.");
    }
}

fn export_to_csv(patients: &[Patient]) {
    let path = "patients.csv";
    match std::fs::File::create(path) {
        Ok(file) => {
            let mut w = csv::Writer::from_writer(file);
            if w.write_record(&["Patient ID", "Name", "Age", "Gender", "Disease", "Medications"])
                .is_err()
            {
                println!("Failed to write header.");
                return;
            }
            for p in patients {
                let meds: String = p
                    .medications
                    .iter()
                    .map(|m| format!("{} ({})", m.name, m.schedule))
                    .collect::<Vec<_>>()
                    .join("; ");
                let _ = w.write_record(&[
                    &p.id,
                    &p.name,
                    &p.age.to_string(),
                    &p.gender,
                    &p.disease,
                    &meds,
                ]);
            }
            println!("✓ Data exported to {}.", path);
        }
        Err(e) => println!("Failed to create CSV: {}", e),
    }
}

fn default_data_path() -> PathBuf {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".into());
    let dir = PathBuf::from(home).join(".meditracker");
    let _ = std::fs::create_dir_all(&dir);
    dir.join("data.json")
}

fn main() {
    let cli = Cli::parse();
    let data_path = cli
        .data_path
        .unwrap_or_else(default_data_path);

    let mut patients = match load_patients(&data_path) {
        Ok(p) => {
            if !p.is_empty() {
                println!("Loaded {} patient(s) from {}.", p.len(), data_path.display());
            }
            p
        }
        Err(e) => {
            eprintln!("Warning: {} Starting with empty data.", e);
            Vec::new()
        }
    };

    loop {
        display_menu();
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_patient(&mut patients),
            "2" => view_patients(&patients),
            "3" => add_medication(&mut patients),
            "4" => view_medications(&patients),
            "5" => remove_medication(&mut patients),
            "6" => update_medication(&mut patients),
            "7" => remove_patient(&mut patients),
            "8" => export_to_csv(&patients),
            "9" => {
                if let Err(e) = save_patients(&data_path, &patients) {
                    eprintln!("Could not save data: {}", e);
                } else if !patients.is_empty() {
                    println!("Data saved. Goodbye!");
                } else {
                    println!("Goodbye!");
                }
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
