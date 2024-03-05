// Crate configuration
#![crate_name = "pdf_ok"]
#![crate_type = "lib"]

use colored::*;
use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// `utils` module contains utility functions used throughout the application.
pub mod utils;
use crate::utils::get_source_front_matter_files;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(
    filename: P,
) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_data(files: Vec<&str>) {
    let mut file = 0;
    let mut total_lines = 0;
    let mut yaml_delimiter_count = 0;
    let mut yaml_content = String::default();
    let mut markdown_content = String::default();

    while file < files.len() {
        // File hosts.txt must exist in the current path
        if let Ok(lines) = read_lines(files[file]) {
            println!("{}", files[file].bright_yellow());
            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(Result::ok) {
                if line.trim() == "---" {
                    yaml_delimiter_count += 1;
                }

                if yaml_delimiter_count == 1
                    && line.trim() != "---"
                    && yaml_delimiter_count < 2
                {
                    yaml_content
                        .push_str(&format!("{}{}", &line, "\n"));
                }
                // else {
                //     println!("{}", line);
                // }

                if yaml_delimiter_count == 2 && line.trim() != "---" {
                    markdown_content
                        .push_str(&format!("{}{}", &line, "\n"));
                }

                total_lines += 1;
            }
        }
        yaml_delimiter_count = 0;
        // // Serialize it to a YAML string.
        // let yaml = serde_yaml::to_string(&yaml_content).expect("Oops");
        // println!("BEFORE: {:#?}", yaml);
        // let deserialized_point: String =
        //     serde_yaml::from_str(&yaml).expect("Oops 2");
        // println!("AFTER: {:#?}", deserialized_point);

        file += 1;
        println!(
            "{}\n{}",
            "Front Matter YAML content:".cyan(),
            yaml_content
        );
        println!(
            "{}\n{}",
            "Markdown content:".cyan(),
            markdown_content
        );
        println!("{} {}\n", "Total lines:".bright_red(), total_lines);
        yaml_content = String::default();
        markdown_content = String::default();
    }
}

/// This is the main entry point for `Rust and Cargo starter (RCS)`
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    // TEST_MODE value can be changed in the .env file in the root of this project
    if env::var("TEST_MODE").unwrap_or_default() == "1" {
        println!(
            "\n{} value: {}",
            "TEST_MODE".bright_cyan(),
            env::var("TEST_MODE").unwrap().to_string().bright_green()
        );
        return Err("Simulated error\n".yellow().into());
    }

    let name = env::var("PROJECT_NAME").unwrap().to_string();
    println!("\nWelcome to {}", name.yellow());
    println!("PDF creation from Front Matter (YAML) files.");

    let fmy_source_files = get_source_front_matter_files();
    println!(
        "{}\n{:#?}",
        "fmy_source_files".to_uppercase().bright_cyan(),
        fmy_source_files
    );

    read_file_data(fmy_source_files);

    Ok(())
}
