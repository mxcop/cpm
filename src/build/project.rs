/// The context of a cpm project.
#[derive(Debug)]
pub(crate) struct Project {
    pub project_dir: String,
    pub source_files: Vec<String>,
    pub include_dir: String,
    pub libraries: Vec<String>
}

impl Project {
    /// Create a new project context.
    pub fn new(project_dir: String, include_dir: String) -> Self {
        Self {
            project_dir,
            include_dir,
            source_files: Vec::new(),
            libraries: Vec::new()
        }
    }
}