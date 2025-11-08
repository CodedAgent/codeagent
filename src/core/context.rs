use std::path::PathBuf;

pub struct ProjectContext {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
}

impl ProjectContext {
    pub fn analyze(root: PathBuf) -> anyhow::Result<Self> {
        let files = vec![];
        Ok(ProjectContext { root, files })
    }
}
