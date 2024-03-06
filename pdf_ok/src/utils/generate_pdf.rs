use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::io::Write;
use std::fs;

use project_root::get_project_root;

pub fn generate_pdf(
    generated_html: String,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let root_path = match get_project_root() {
        Ok(p) => format!("{:?}", p),
        Err(e) => format!("{:?}", e),
    };
    let pdf_path = format!("{}.pdf", filename);
    let mut html = String::new();
    // url_escape:: comes from the url_escape crate
    url_escape::encode_query_to_string(generated_html, &mut html);

    println!("root_path: {}", root_path);

    tab.navigate_to(
        format!("data:text/html;charset=utf-8,{}", html)
            .as_str(),
    )?;
    let _jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true,
    )?;

    let pdf_ = tab.print_to_pdf(None)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(pdf_path)?;
    let _ = file.write_all(&pdf_);
    Ok(())
}