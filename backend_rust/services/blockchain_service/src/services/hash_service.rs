//! Hash computation service for content verification

use anyhow::Result;
use sha2::{Digest, Sha256};

/// Compute SHA-256 hash of content
pub fn compute_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Hash service for content verification
#[derive(Clone)]
pub struct HashService;

impl HashService {
    /// Create a new hash service
    pub fn new() -> Self {
        HashService
    }

    /// Compute hash and measure time
    pub fn compute_hash_with_timing(&self, content: &[u8]) -> crate::models::HashResult {
        let start = std::time::Instant::now();
        let hash = compute_hash(content);
        let elapsed = start.elapsed().as_millis() as u64;

        crate::models::HashResult::new(hash, elapsed, content.len() as u64)
    }

    /// Verify that content matches a given hash
    pub fn verify_hash(&self, content: &[u8], expected_hash: &str) -> bool {
        let computed = compute_hash(content);
        computed == expected_hash
    }
}

impl Default for HashService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let content = b"Hello, World!";
        let hash = compute_hash(content);
        
        // SHA-256 of "Hello, World!" is known
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_hash_service() {
        let service = HashService::new();
        let content = b"Test content";
        
        let result = service.compute_hash_with_timing(content);
        
        assert!(!result.hash.is_empty());
        assert_eq!(result.content_size, content.len() as u64);
        assert!(result.computation_time_ms < 10000); // Should be much faster
    }

    #[test]
    fn test_verify_hash() {
        let service = HashService::new();
        let content = b"Hello, World!";
        let correct_hash = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
        let wrong_hash = "abc123";
        
        assert!(service.verify_hash(content, correct_hash));
        assert!(!service.verify_hash(content, wrong_hash));
    }
}
