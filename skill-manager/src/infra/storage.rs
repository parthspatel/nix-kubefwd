//! File storage implementations

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::SkillScope;
use crate::services::{OutputStorage, SkillStorage};
use crate::utils::error::{Error, Result};
use crate::utils::hash;

/// File system based skill storage
pub struct FileSkillStorage {
    base_path: PathBuf,
}

impl FileSkillStorage {
    /// Create a new skill storage
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Get the directory for a skill
    fn skill_dir(&self, skill_id: Uuid) -> PathBuf {
        self.base_path.join("skills").join(skill_id.to_string())
    }

    /// Get the path to a skill's CLAUDE.md file
    fn skill_file(&self, skill_id: Uuid) -> PathBuf {
        self.skill_dir(skill_id).join("CLAUDE.md")
    }
}

#[async_trait]
impl SkillStorage for FileSkillStorage {
    async fn store(&self, skill_id: Uuid, content: &str) -> Result<String> {
        let dir = self.skill_dir(skill_id);
        let file = self.skill_file(skill_id);

        // Create directory
        tokio::fs::create_dir_all(&dir)
            .await
            .map_err(|e| Error::Io(e))?;

        // Write content
        tokio::fs::write(&file, content)
            .await
            .map_err(|e| Error::Io(e))?;

        // Calculate and return hash
        Ok(self.hash_content(content))
    }

    async fn read(&self, skill_id: Uuid) -> Result<String> {
        let file = self.skill_file(skill_id);

        if !file.exists() {
            return Err(Error::FileNotFound(file));
        }

        tokio::fs::read_to_string(&file)
            .await
            .map_err(|e| Error::Io(e))
    }

    async fn delete(&self, skill_id: Uuid) -> Result<()> {
        let dir = self.skill_dir(skill_id);

        if dir.exists() {
            tokio::fs::remove_dir_all(&dir)
                .await
                .map_err(|e| Error::Io(e))?;
        }

        Ok(())
    }

    async fn exists(&self, skill_id: Uuid) -> Result<bool> {
        Ok(self.skill_file(skill_id).exists())
    }

    fn get_path(&self, skill_id: Uuid) -> PathBuf {
        self.skill_file(skill_id)
    }

    fn hash_content(&self, content: &str) -> String {
        hash::sha256(content)
    }
}

/// File system based output storage
pub struct FileOutputStorage {
    csm_home: PathBuf,
    claude_home: PathBuf,
}

impl FileOutputStorage {
    /// Create a new output storage
    pub fn new(csm_home: impl Into<PathBuf>) -> Self {
        let csm_home = csm_home.into();

        // Claude home is typically ~/.claude/
        let claude_home = csm_home
            .parent()
            .map(|p| p.join(".claude"))
            .unwrap_or_else(|| PathBuf::from(".claude"));

        Self { csm_home, claude_home }
    }

    /// Create with explicit Claude home path
    pub fn with_claude_home(csm_home: impl Into<PathBuf>, claude_home: impl Into<PathBuf>) -> Self {
        Self {
            csm_home: csm_home.into(),
            claude_home: claude_home.into(),
        }
    }
}

#[async_trait]
impl OutputStorage for FileOutputStorage {
    async fn write_claude_md(&self, scope: &SkillScope, content: &str) -> Result<()> {
        let path = self.get_claude_md_path(scope);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::Io(e))?;
        }

        tokio::fs::write(&path, content)
            .await
            .map_err(|e| Error::Io(e))
    }

    async fn read_claude_md(&self, scope: &SkillScope) -> Result<Option<String>> {
        let path = self.get_claude_md_path(scope);

        if !path.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| Error::Io(e))?;

        Ok(Some(content))
    }

    fn get_claude_md_path(&self, scope: &SkillScope) -> PathBuf {
        match scope {
            SkillScope::Global => self.claude_home.join("CLAUDE.md"),
            SkillScope::Project { path } => path.join("CLAUDE.md"),
        }
    }

    async fn create_symlinks(&self, project_path: &Path, skill_ids: &[Uuid]) -> Result<()> {
        let csm_dir = project_path.join(".csm").join("skills");

        // Create .csm/skills directory
        tokio::fs::create_dir_all(&csm_dir)
            .await
            .map_err(|e| Error::Io(e))?;

        // Create symlinks for each skill
        for skill_id in skill_ids {
            let source = self.csm_home.join("skills").join(skill_id.to_string());
            let target = csm_dir.join(skill_id.to_string());

            // Remove existing symlink if present
            if target.exists() || target.is_symlink() {
                tokio::fs::remove_file(&target).await.ok();
            }

            // Create symlink
            #[cfg(unix)]
            {
                tokio::fs::symlink(&source, &target)
                    .await
                    .map_err(|e| Error::Io(e))?;
            }

            #[cfg(windows)]
            {
                // Windows requires admin or developer mode for symlinks
                // Fall back to junction or copy
                std::os::windows::fs::symlink_dir(&source, &target)
                    .map_err(|e| Error::Io(e))?;
            }
        }

        Ok(())
    }

    async fn remove_symlinks(&self, project_path: &Path) -> Result<()> {
        let csm_dir = project_path.join(".csm").join("skills");

        if csm_dir.exists() {
            tokio::fs::remove_dir_all(&csm_dir)
                .await
                .map_err(|e| Error::Io(e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_skill_storage_crud() {
        let temp = tempdir().unwrap();
        let storage = FileSkillStorage::new(temp.path());

        let skill_id = Uuid::new_v4();
        let content = "# Test Skill\n\nSome content.";

        // Store
        let hash = storage.store(skill_id, content).await.unwrap();
        assert!(!hash.is_empty());

        // Exists
        assert!(storage.exists(skill_id).await.unwrap());

        // Read
        let retrieved = storage.read(skill_id).await.unwrap();
        assert_eq!(retrieved, content);

        // Delete
        storage.delete(skill_id).await.unwrap();
        assert!(!storage.exists(skill_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_skill_storage_hash_consistency() {
        let temp = tempdir().unwrap();
        let storage = FileSkillStorage::new(temp.path());

        let content = "Test content";
        let hash1 = storage.hash_content(content);
        let hash2 = storage.hash_content(content);

        assert_eq!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_output_storage_write_read() {
        let temp = tempdir().unwrap();
        let csm_home = temp.path().join(".csm");
        let claude_home = temp.path().join(".claude");

        let storage = FileOutputStorage::with_claude_home(&csm_home, &claude_home);

        let content = "# Merged Content";
        storage.write_claude_md(&SkillScope::Global, content).await.unwrap();

        let retrieved = storage.read_claude_md(&SkillScope::Global).await.unwrap();
        assert_eq!(retrieved, Some(content.to_string()));
    }

    #[tokio::test]
    async fn test_output_storage_project_path() {
        let temp = tempdir().unwrap();
        let storage = FileOutputStorage::new(temp.path());

        let project_path = temp.path().join("my-project");
        let scope = SkillScope::Project { path: project_path.clone() };

        let path = storage.get_claude_md_path(&scope);
        assert_eq!(path, project_path.join("CLAUDE.md"));
    }
}
