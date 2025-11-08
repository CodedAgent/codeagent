use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CacheKey {
    pub category: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: u64,
    pub ttl_seconds: u64,
}

pub struct Cache<T> {
    storage: HashMap<CacheKey, CacheEntry<T>>,
    max_entries: usize,
}

impl<T: Clone> Cache<T> {
    pub fn new(max_entries: usize) -> Self {
        Cache {
            storage: HashMap::new(),
            max_entries,
        }
    }

    pub fn get(&self, key: &CacheKey) -> Option<T> {
        if let Some(entry) = self.storage.get(key) {
            if self.is_valid(entry) {
                return Some(entry.data.clone());
            }
        }
        None
    }

    pub fn set(&mut self, key: CacheKey, data: T, ttl_seconds: u64) {
        if self.storage.len() >= self.max_entries {
            self.evict_oldest();
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.storage.insert(
            key,
            CacheEntry {
                data,
                timestamp,
                ttl_seconds,
            },
        );
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }

    pub fn cleanup_expired(&mut self) {
        let expired_keys: Vec<_> = self
            .storage
            .iter()
            .filter(|(_, entry)| !self.is_valid(entry))
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.storage.remove(&key);
        }
    }

    fn is_valid(&self, entry: &CacheEntry<T>) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        (now - entry.timestamp) < entry.ttl_seconds
    }

    fn evict_oldest(&mut self) {
        if let Some((key, _)) = self
            .storage
            .iter()
            .min_by_key(|(_, entry)| entry.timestamp)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            self.storage.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let mut cache: Cache<String> = Cache::new(10);
        
        let key = CacheKey {
            category: "test".to_string(),
            identifier: "key1".to_string(),
        };

        cache.set(key.clone(), "value".to_string(), 3600);
        assert_eq!(cache.get(&key), Some("value".to_string()));
    }
}
