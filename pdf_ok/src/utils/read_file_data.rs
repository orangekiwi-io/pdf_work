use colored::*;
use serde::Deserialize;
use serde_yaml::{to_string as yaml_to_string, Value};
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
        // Deserialize from a YAML string
        let de = serde_yaml::Deserializer::from_str(&yaml_content);
        // Extract the data value from the deserialized result
        let value = Value::deserialize(de);
        // Convert the serde_yaml Value type to something we can use (in this case a string)
        // We are looking for the author YAML data
        // TODO RL Error handing for when the 'get' key/index does not exist
        let author_value =
            yaml_to_string(value.unwrap().get("author").unwrap())
                .expect("msg");
        println!(
            "{} {}\n",
            "Deserializer author YAML value:".cyan(),
            author_value.trim().bright_white()
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
