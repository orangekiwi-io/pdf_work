use colored::*;
use serde_yaml::{to_string as yaml_to_string, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::utils::generate_pdf;

/// Reads data from Markdown files, extracts YAML front matter, and generates PDF files.
///
/// # Arguments
///
/// * `files` - A vector of file paths to Markdown files containing Front Mattter YAML.
///
/// # Examples
///
/// ```
/// use your_crate_name::read_file_data;
///
/// let files = vec!["./source_files/404.md"];
/// read_file_data(files);
/// ```
pub fn read_file_data(files: Vec<&str>) {
    let mut file = 0;
    let mut yaml_delimiter_count = 0;
    let mut yaml_content: String = String::default();
    let mut markdown_content: String = String::default();

    while file < files.len() {
        let filename = files[file];
        if let Ok(lines) = read_lines(filename) {
            println!("{}", filename.bright_yellow());
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

                if yaml_delimiter_count == 2 && line.trim() != "---" {
                    markdown_content
                        .push_str(&format!("{}{}", &line, "\n"));
                }
            }
        }
        yaml_delimiter_count = 0;

        let yaml: Value = serde_yaml::from_str(&yaml_content).unwrap();
        let yaml_btreemap: BTreeMap<String, Value> =
            yaml_mapping_to_btreemap(&yaml).unwrap();
        println!(
            "{} {:#?}\n",
            "yaml_btreemap value:".cyan(),
            yaml_btreemap
        );
        println!(
            "{} {:?}",
            "subtitle:".cyan(),
            yaml_to_string(yaml_btreemap.get("subtitle").unwrap())
                .unwrap()
                .trim()
        );

        // Convert Markdown content to HTML
        // markdown:: comes from the markdown crate
        let html: String =
            markdown::to_html(&markdown_content.to_owned());

        // Remove the markdown, md, file extension
        let filename_path = filename.trim_end_matches(".md");
        let _ = generate_pdf(html, filename_path);

        file += 1;
        // Reset yaml and markdown content ready for the next file
        yaml_content = String::default();
        markdown_content = String::default();
    }
}

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

/// Converts a YAML mapping into a Rust BTreeMap with string keys and arbitrary values. BTreeMaps are automatically alphabetically sorted.
///
/// # Arguments
///
/// * `yaml` - A reference to a serde_yaml::Value containing the YAML data to convert.
///
/// # Returns
///
/// An Option containing the resulting BTreeMap<String, Value>. If the YAML data is successfully
/// converted into a BTreeMap, it returns Some(btreemap); otherwise, it returns None.
///
/// # Examples
///
/// ```
/// use serde_yaml::Value;
/// use std::collections::BTreeMap;
///
/// let yaml_data = serde_yaml::from_str("name: John\nage: 30").unwrap();
/// let btreemap = yaml_mapping_to_btreemap(&yaml_data).unwrap();
/// assert_eq!(btreemap.get("name"), Some(&Value::String("John".to_string())));
/// assert_eq!(btreemap.get("age"), Some(&Value::Number(30.into())));
/// ```
fn yaml_mapping_to_btreemap(
    yaml: &Value,
) -> Option<BTreeMap<String, Value>> {
    match yaml {
        // Match if yaml Value contains a Mapping 'object'
        Value::Mapping(mapping_value) => {
            // Create a new BTreeMap to hold the YAML data
            let mut yaml_btreemap: BTreeMap<String, Value> =
                BTreeMap::new();

            // Iterate over key-value pairs in the mapping
            for (key, value) in mapping_value.iter() {
                // Destructure the key-value tuple, if the key is of type Value::String.
                if let (Value::String(key), value) = (key, value) {
                    // Insert key-value pair into the BTreeMap. The key and value values are cloned because or_insert takes ownership of the arguments
                    yaml_btreemap
                        .entry(key.clone())
                        .or_insert(value.clone());
                } else {
                    // Handle non-string keys or non-scalar values
                    return None;
                }
            }

            println!("{} {:#?}", "yaml_btreemap".cyan(), yaml_btreemap);
            // Return the resulting BTreeMap
            Some(yaml_btreemap)
        }
        _ => None, // Return None if yaml is not a mapping
    }
}
