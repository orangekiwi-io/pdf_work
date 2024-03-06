// Copyright © 2024 PDF OK (pdf_ok). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Crate configuration
#![crate_name = "pdf_ok"]
#![crate_type = "lib"]

use colored::*; // Import the colored crate for colourful console output
use dotenvy::dotenv; // Import the dotenvy crate for loading environment variables from a .env file
use std::env;

/// `utils` module contains utility functions used throughout the application.
pub mod utils;
use crate::utils::{get_source_front_matter_files, read_file_data};

/// This is the main entry point for the PDF from Front Matter YAML package/crate.
///
/// # Errors
///
/// Returns a boxed error if there is an issue reading the .env file or if the TEST_MODE is set to "1".
///
/// # Examples
///
/// ```
/// use pdf_ok::run;
///
/// let result = run();
/// assert!(result.is_ok());
/// ```
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found"); // Load environment variables from .env file

    // Check if TEST_MODE is set to "1" in the .env file
    if env::var("TEST_MODE").unwrap_or_default() == "1" {
        println!(
            "\n{} value: {}",
            "TEST_MODE".bright_cyan(),
            env::var("TEST_MODE").unwrap().to_string().bright_green()
        );
        // Simulate an error in TEST_MODE by returning a boxed error
        return Err("Simulated error\n".yellow().into());
    }

    // Fetch the project name from the environment variables
    let name = env::var("PROJECT_NAME").unwrap().to_string();
    println!("\nWelcome to {}", name.yellow());
    println!("PDF creation from Front Matter (YAML) files.");

    // Retrieve Front Matter YAML source files and read their data
    let fmy_source_files = get_source_front_matter_files();
    read_file_data(fmy_source_files);

    Ok(())
}
