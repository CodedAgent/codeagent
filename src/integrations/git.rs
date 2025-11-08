use anyhow::Result;
use std::path::Path;
use git2::{Repository, IndexAddOption};

pub struct GitManager {
    repo: Option<Repository>,
}

impl GitManager {
    pub fn new(repo_path: &Path) -> Result<Self> {
        let repo = Repository::open(repo_path).ok();
        Ok(GitManager { repo })
    }

    pub fn is_git_repo(&self) -> bool {
        self.repo.is_some()
    }

    pub fn stage_changes(&self, patterns: &[&str]) -> Result<()> {
        if let Some(repo) = &self.repo {
            let mut index = repo.index()?;
            index.add_all(patterns.iter().map(|s| s.to_string()), IndexAddOption::DEFAULT, None)?;
            index.write()?;
            tracing::info!("Staged changes: {:?}", patterns);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Not a Git repository"))
        }
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        if let Some(repo) = &self.repo {
            let signature = repo.signature()?;
            let tree_id = {
                let mut index = repo.index()?;
                index.write_tree()?
            };
            let tree = repo.find_tree(tree_id)?;
            let parent_commit = repo.head()?.peel_to_commit()?;
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[&parent_commit],
            )?;
            tracing::info!("Committed with message: {}", message);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Not a Git repository"))
        }
    }

    pub fn get_status(&self) -> Result<Vec<String>> {
        if let Some(repo) = &self.repo {
            let statuses = repo.statuses(None)?;
            let mut status_list = Vec::new();

            for item in statuses.iter() {
                if let Some(path) = item.path() {
                    status_list.push(path.to_string());
                }
            }

            Ok(status_list)
        } else {
            Err(anyhow::anyhow!("Not a Git repository"))
        }
    }
}
