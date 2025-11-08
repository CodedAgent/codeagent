use super::storage::{Cache, CacheKey};
use crate::parsers::{TestResult, LintResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedAnalysis {
    pub file_path: String,
    pub test_results: Vec<TestResult>,
    pub lint_results: Vec<LintResult>,
    pub analysis_timestamp: u64,
}

pub struct AnalysisCache {
    cache: Cache<CachedAnalysis>,
}

impl AnalysisCache {
    pub fn new(max_entries: usize) -> Self {
        AnalysisCache {
            cache: Cache::new(max_entries),
        }
    }

    pub fn get_analysis(&self, file_path: &str) -> Option<CachedAnalysis> {
        let key = CacheKey {
            category: "analysis".to_string(),
            identifier: file_path.to_string(),
        };
        self.cache.get(&key)
    }

    pub fn cache_analysis(&mut self, analysis: CachedAnalysis, ttl_hours: u64) {
        let key = CacheKey {
            category: "analysis".to_string(),
            identifier: analysis.file_path.clone(),
        };
        self.cache.set(key, analysis, ttl_hours * 3600);
    }

    pub fn invalidate(&mut self, file_path: &str) {
        self.cache.cleanup_expired();
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache.size()
    }
}
