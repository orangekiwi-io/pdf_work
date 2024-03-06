use headless_chrome::Browser;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::utils::extract_to_end_string;
/// Generates a PDF from HTML content using headless Chrome.
///
/// # Arguments
///
/// * `generated_html` - The HTML content to convert to PDF.
/// * `filename` - The name of the PDF file to generate.
///
/// # Errors
///
/// Returns a boxed error if there is an issue with the headless Chrome browser, navigation,
/// capturing screenshot, printing to PDF, or writing the PDF file.
///
/// # Examples
///
/// ```
/// use your_crate_name::generate_pdf;
///
/// let generated_html = "<html><body><h1>Hello, world!</h1></body></html>".to_string();
/// let filename = "example";
/// let result = generate_pdf(generated_html, filename);
/// assert!(result.is_ok());
/// ```
pub fn generate_pdf(
    generated_html: String,
    filename_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?; // Start a new headless Chrome browser instance
    let tab = browser.new_tab()?; // Open a new tab

    let mut html = String::new();
    // Encode the HTML content to URL-safe format
    // url_escape:: comes from the url_escape crate
    url_escape::encode_query_to_string(generated_html, &mut html);

    // TODO RL Allow path to be set by the user, keeping "pdfs" as a fallback/default location
    let output_directory = "pdfs";
    fs::create_dir_all(output_directory)?;
    let extracted_filename = extract_to_end_string(filename_path, '/');
    let mut pdf_file = extracted_filename.unwrap().to_string();
    pdf_file.push_str(".pdf");

    let pdf_file_path = Path::new(output_directory).join(pdf_file);

    // Navigate the tab to the HTML content.
    // In this case, the page is a data stream
    tab.navigate_to(
        format!("data:text/html;charset=utf-8,{}", html)
            .as_str(),
    )?;

    // Convert the page to PDF format
    let pdf = tab.print_to_pdf(None)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(pdf_file_path)?;
    let _ = file.write_all(&pdf);

    Ok(())
}
