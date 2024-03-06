/// Retrieves a list of source Front Matter YAML files.
///
/// # Example
///
/// ```
/// use your_crate_name::get_source_front_matter_files;
///
/// let files = get_source_front_matter_files();
/// assert_eq!(files.len(), 2); // Ensure that two files are returned
/// assert_eq!(files[0], "./source_files/404.md"); // Check the first file path
/// assert_eq!(files[1], "./source_files/features.md"); // Check the second file path
/// ```
pub fn get_source_front_matter_files() -> Vec<&'static str> {
    vec![
        "./source_files/404.md",
        "./source_files/features.md",
    ]
}
