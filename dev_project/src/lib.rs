#![crate_type = "lib"]

use colored::Colorize;
use dotenvy::dotenv;
use pdf_composer::{PDFComposer, PDFDocInfoEntry};
use std::{env, path::PathBuf};

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
    println!("Used as test project to test PDF creation from Front Matter (YAML) files.\n");

    // Create a new PDFComposer instance
    let mut bob = PDFComposer::new();

    // Add some paths. Relative paths
    let paths = vec![
        PathBuf::from("source_mds/untitled.md"),
        PathBuf::from("source_mds/file_02.md"),
        PathBuf::from("source_mds/file_not_found.md"),
        PathBuf::from("source_mds/untitled.txt"),

    ];
    bob.add_source_files(paths);

    // PDF version (not the version of the document, but the Adobe (formerly) PDF format version)
    bob.set_pdf_version("2.0");

    // Output directory for the generated PDFs
    bob.set_output_directory("output_pdfs");

    // Metadata for the PDFs
    // Title property set via the HTML template <title> tag
    let author_entry = PDFDocInfoEntry {
        doc_info_entry: "AuthoR".to_owned(),
        yaml_entry: "author".to_owned(),
    };
    let keywords_entry = PDFDocInfoEntry {
        doc_info_entry: "Keywords".to_owned(),
        yaml_entry: "keywords".to_owned(),
    };
    let subject_entry = PDFDocInfoEntry {
        doc_info_entry: "Subject".to_owned(),
        yaml_entry: "description".to_owned(),
    };
    let language_entry = PDFDocInfoEntry {
        doc_info_entry: "Language".to_owned(),
        yaml_entry: "language".to_owned(),
    };
    bob.set_doc_info_entry(author_entry);
    bob.set_doc_info_entry(keywords_entry);
    bob.set_doc_info_entry(subject_entry);
    bob.set_doc_info_entry(language_entry);

    // Generate the PDFs
    bob.generate_pdfs();

    Ok(())
}
