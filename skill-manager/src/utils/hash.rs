//! Hash utilities for content verification

use sha2::{Digest, Sha256};

/// Calculate SHA-256 hash of content
pub fn sha256(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Calculate SHA-256 hash and return first 12 characters (short hash)
pub fn sha256_short(content: &str) -> String {
    sha256(content).chars().take(12).collect()
}

// Inline hex encoding to avoid another dependency
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let hash = sha256("hello world");
        assert_eq!(hash.len(), 64); // SHA-256 produces 32 bytes = 64 hex chars
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_sha256_short() {
        let hash = sha256_short("hello world");
        assert_eq!(hash.len(), 12);
        assert_eq!(hash, "b94d27b9934d");
    }

    #[test]
    fn test_sha256_deterministic() {
        let content = "test content";
        assert_eq!(sha256(content), sha256(content));
    }

    #[test]
    fn test_sha256_different_content() {
        assert_ne!(sha256("content1"), sha256("content2"));
    }
}
