use std::path::{Path, PathBuf};
use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file: PathBuf,
    pub line_number: usize,
    pub content: String,
    pub match_position: usize,
}

pub struct SemanticSearch;

impl SemanticSearch {
    pub fn search_pattern(
        pattern: &str,
        files: &[PathBuf],
    ) -> Result<Vec<SearchResult>> {
        let regex = Regex::new(pattern)?;
        let mut results = Vec::new();

        for file_path in files {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(mat) = regex.find(line) {
                        results.push(SearchResult {
                            file: file_path.clone(),
                            line_number: line_num + 1,
                            content: line.to_string(),
                            match_position: mat.start(),
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    pub fn search_keyword(query: &str, files: &[PathBuf]) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        for file_path in files {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                for (line_num, line) in content.lines().enumerate() {
                    if line.contains(query) {
                        if let Some(pos) = line.find(query) {
                            results.push(SearchResult {
                                file: file_path.clone(),
                                line_number: line_num + 1,
                                content: line.to_string(),
                                match_position: pos,
                            });
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}
