/// Retrieves a list of source front matter files.
///
/// # Example
///
/// ```
/// use your_crate_name::get_source_front_matter_files;
///
/// let files = get_source_front_matter_files();
/// assert_eq!(files.len(), 1);
/// assert_eq!(files[0], "./source_files/404.md");
/// ```
pub fn get_source_front_matter_files() -> Vec<&'static str> {
    vec![
        "./source_files/404.md",
        "./source_files/features.md",
    ]
}
