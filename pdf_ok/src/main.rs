// Copyright © 2024 Rust and Cargo starter (RCS). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is the main entry point for the PDF from Front Matter YAML package/crate.
use dotenvy::dotenv;
use std::env;

fn main() {
  dotenv().expect(".env file not found");
  let project_name = env::var("PROJECT_NAME").unwrap().to_string();

  // Call the `run()` function from the pdf_ok module.
  if let Err(err) = pdf_ok::run() {
      eprintln!("Error running {}: {}", project_name, err);
      std::process::exit(1);
  }
}
