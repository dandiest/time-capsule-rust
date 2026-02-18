use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct TimeCapsule {
    message: String,
    timestamp: DateTime<Utc>,
}

fn main() {
    let file_path = "capsule.json";

    match fs::read_to_string(file_path) {
        Ok(content) => {
            let old_capsule: TimeCapsule = serde_json::from_str(&content).expect("Error");

            println!("Old message found (of {})", old_capsule.timestamp);
            println!("> {}\n", old_capsule.message);
        }
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                println!("No previous capsules found. We're creating another one.");
            } else {
                eprintln!("Unexpected error reading: {}", error);
                return;
            }
        }
    }

    print!("Please, insert new capsule message.");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let new_capsule = TimeCapsule {
        message: input.trim().to_string(),
        timestamp: Utc::now(),
    };

    let json = serde_json::to_string_pretty(&new_capsule).expect("Error during serialization.");
    fs::write(file_path, json).expect("Error");
    println!("Successfully updated capsule!");
}
