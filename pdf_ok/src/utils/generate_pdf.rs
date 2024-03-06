use headless_chrome::Browser;
use std::io::Write;
use std::fs;

use project_root::get_project_root;

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
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?; // Start a new headless Chrome browser instance
    let tab = browser.new_tab()?; // Open a new tab
    let root_path = match get_project_root() { // Get the root path of the project
        Ok(p) => format!("{:?}", p),
        Err(e) => format!("{:?}", e),
    };
    let pdf_path = format!("{}.pdf", filename); // Create the PDF file path
    let mut html = String::new();
    // Encode the HTML content to URL-safe format
    // url_escape:: comes from the url_escape crate
    url_escape::encode_query_to_string(generated_html, &mut html);

    println!("root_path: {}", root_path);

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
        .open(pdf_path)?;
    let _ = file.write_all(&pdf);
    
    Ok(())
}