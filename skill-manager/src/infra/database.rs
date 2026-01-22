//! Database implementations (SQLite)

use std::path::Path;
use std::sync::Mutex;

use async_trait::async_trait;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use crate::domain::{Conflict, ConflictStatus, ConflictType, Skill, SkillScope, SkillSource};
use crate::services::{ConflictRepository, SkillRepository};
use crate::utils::error::{Error, Result};

/// SQLite-based skill repository
pub struct SqliteSkillRepository {
    conn: Mutex<Connection>,
}

impl SqliteSkillRepository {
    /// Create a new repository with the given database path
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let repo = Self { conn: Mutex::new(conn) };
        repo.init_schema()?;
        Ok(repo)
    }

    /// Create an in-memory repository (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let repo = Self { conn: Mutex::new(conn) };
        repo.init_schema()?;
        Ok(repo)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS skills (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                source_json TEXT NOT NULL,
                scope_json TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                content_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                tags_json TEXT NOT NULL DEFAULT '[]',
                priority INTEGER NOT NULL DEFAULT 50,
                update_mode TEXT NOT NULL DEFAULT 'auto'
            );

            CREATE INDEX IF NOT EXISTS idx_skills_name ON skills(name);
            CREATE INDEX IF NOT EXISTS idx_skills_enabled ON skills(enabled);
            "#,
        )?;

        Ok(())
    }

    /// Convert a row to a Skill
    fn row_to_skill(row: &rusqlite::Row) -> rusqlite::Result<Skill> {
        use chrono::{DateTime, Utc};

        let id: String = row.get(0)?;
        let source_json: String = row.get(3)?;
        let scope_json: String = row.get(4)?;
        let tags_json: String = row.get(9)?;
        let created_at_str: String = row.get(7)?;
        let updated_at_str: String = row.get(8)?;

        Ok(Skill {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            name: row.get(1)?,
            description: row.get(2)?,
            source: serde_json::from_str(&source_json).unwrap_or(SkillSource::Inline),
            scope: serde_json::from_str(&scope_json).unwrap_or(SkillScope::Global),
            enabled: row.get::<_, i32>(5)? != 0,
            content_hash: row.get(6)?,
            created_at: DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            tags: serde_json::from_str(&tags_json).unwrap_or_default(),
            priority: row.get(10)?,
            update_mode: row.get::<_, String>(11)?.parse().unwrap_or_default(),
        })
    }
}

#[async_trait]
impl SkillRepository for SqliteSkillRepository {
    async fn create(&self, skill: &Skill) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let source_json = serde_json::to_string(&skill.source)?;
        let scope_json = serde_json::to_string(&skill.scope)?;
        let tags_json = serde_json::to_string(&skill.tags)?;

        conn.execute(
            r#"
            INSERT INTO skills (id, name, description, source_json, scope_json, enabled,
                               content_hash, created_at, updated_at, tags_json, priority, update_mode)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            params![
                skill.id.to_string(),
                skill.name,
                skill.description,
                source_json,
                scope_json,
                skill.enabled as i32,
                skill.content_hash,
                skill.created_at.to_rfc3339(),
                skill.updated_at.to_rfc3339(),
                tags_json,
                skill.priority,
                skill.update_mode.to_string(),
            ],
        )?;

        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<Skill>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let skill = conn
            .query_row(
                "SELECT * FROM skills WHERE id = ?1",
                params![id.to_string()],
                Self::row_to_skill,
            )
            .optional()?;

        Ok(skill)
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<Skill>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let skill = conn
            .query_row(
                "SELECT * FROM skills WHERE name = ?1",
                params![name],
                Self::row_to_skill,
            )
            .optional()?;

        Ok(skill)
    }

    async fn update(&self, skill: &Skill) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let source_json = serde_json::to_string(&skill.source)?;
        let scope_json = serde_json::to_string(&skill.scope)?;
        let tags_json = serde_json::to_string(&skill.tags)?;

        conn.execute(
            r#"
            UPDATE skills SET
                name = ?2, description = ?3, source_json = ?4, scope_json = ?5,
                enabled = ?6, content_hash = ?7, updated_at = ?8, tags_json = ?9,
                priority = ?10, update_mode = ?11
            WHERE id = ?1
            "#,
            params![
                skill.id.to_string(),
                skill.name,
                skill.description,
                source_json,
                scope_json,
                skill.enabled as i32,
                skill.content_hash,
                skill.updated_at.to_rfc3339(),
                tags_json,
                skill.priority,
                skill.update_mode.to_string(),
            ],
        )?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;
        conn.execute("DELETE FROM skills WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Skill>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let mut stmt = conn.prepare("SELECT * FROM skills ORDER BY priority DESC, name ASC")?;
        let skills = stmt
            .query_map([], Self::row_to_skill)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(skills)
    }

    async fn list_by_scope(&self, scope: &SkillScope) -> Result<Vec<Skill>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;
        let scope_json = serde_json::to_string(scope)?;

        let mut stmt = conn.prepare(
            "SELECT * FROM skills WHERE scope_json = ?1 ORDER BY priority DESC, name ASC",
        )?;
        let skills = stmt
            .query_map(params![scope_json], Self::row_to_skill)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(skills)
    }

    async fn list_enabled(&self) -> Result<Vec<Skill>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT * FROM skills WHERE enabled = 1 ORDER BY priority DESC, name ASC",
        )?;
        let skills = stmt
            .query_map([], Self::row_to_skill)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(skills)
    }

    async fn search(&self, query: &str) -> Result<Vec<Skill>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT * FROM skills WHERE name LIKE ?1 OR description LIKE ?1 OR tags_json LIKE ?1 ORDER BY name ASC",
        )?;
        let skills = stmt
            .query_map(params![pattern], Self::row_to_skill)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(skills)
    }

    async fn exists(&self, name: &str) -> Result<bool> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM skills WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }
}

/// SQLite-based conflict repository
pub struct SqliteConflictRepository {
    conn: Mutex<Connection>,
}

impl SqliteConflictRepository {
    /// Create a new repository with the given database path
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let repo = Self { conn: Mutex::new(conn) };
        repo.init_schema()?;
        Ok(repo)
    }

    /// Create an in-memory repository (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let repo = Self { conn: Mutex::new(conn) };
        repo.init_schema()?;
        Ok(repo)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS conflicts (
                id TEXT PRIMARY KEY,
                skill_a_id TEXT NOT NULL,
                skill_b_id TEXT NOT NULL,
                conflict_type TEXT NOT NULL,
                description TEXT NOT NULL,
                line_a INTEGER,
                line_b INTEGER,
                content_a TEXT,
                content_b TEXT,
                suggestion TEXT,
                status TEXT NOT NULL DEFAULT 'unresolved',
                detected_at TEXT NOT NULL,
                resolved_at TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_conflicts_status ON conflicts(status);
            "#,
        )?;

        Ok(())
    }

    /// Convert a row to a Conflict
    fn row_to_conflict(row: &rusqlite::Row) -> rusqlite::Result<Conflict> {
        use chrono::{DateTime, Utc};

        let id: String = row.get(0)?;
        let skill_a_id: String = row.get(1)?;
        let skill_b_id: String = row.get(2)?;
        let conflict_type: String = row.get(3)?;
        let status: String = row.get(10)?;
        let detected_at_str: String = row.get(11)?;
        let resolved_at_str: Option<String> = row.get(12)?;

        Ok(Conflict {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            skill_a_id: Uuid::parse_str(&skill_a_id).unwrap_or_default(),
            skill_b_id: Uuid::parse_str(&skill_b_id).unwrap_or_default(),
            conflict_type: match conflict_type.as_str() {
                "duplicate" => ConflictType::Duplicate,
                "contradictory" => ConflictType::Contradictory,
                "overlap" => ConflictType::Overlap,
                _ => ConflictType::Structural,
            },
            description: row.get(4)?,
            line_a: row.get(5)?,
            line_b: row.get(6)?,
            content_a: row.get(7)?,
            content_b: row.get(8)?,
            suggestion: row.get(9)?,
            status: match status.as_str() {
                "resolved" => ConflictStatus::Resolved,
                "ignored" => ConflictStatus::Ignored,
                _ => ConflictStatus::Unresolved,
            },
            detected_at: DateTime::parse_from_rfc3339(&detected_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            resolved_at: resolved_at_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .ok()
            }),
        })
    }
}

#[async_trait]
impl ConflictRepository for SqliteConflictRepository {
    async fn create(&self, conflict: &Conflict) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        conn.execute(
            r#"
            INSERT INTO conflicts (id, skill_a_id, skill_b_id, conflict_type, description,
                                  line_a, line_b, content_a, content_b, suggestion,
                                  status, detected_at, resolved_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
            params![
                conflict.id.to_string(),
                conflict.skill_a_id.to_string(),
                conflict.skill_b_id.to_string(),
                format!("{:?}", conflict.conflict_type).to_lowercase(),
                conflict.description,
                conflict.line_a,
                conflict.line_b,
                conflict.content_a,
                conflict.content_b,
                conflict.suggestion,
                conflict.status.to_string(),
                conflict.detected_at.to_rfc3339(),
                conflict.resolved_at.map(|dt| dt.to_rfc3339()),
            ],
        )?;

        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<Conflict>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let conflict = conn
            .query_row(
                "SELECT * FROM conflicts WHERE id = ?1",
                params![id.to_string()],
                Self::row_to_conflict,
            )
            .optional()?;

        Ok(conflict)
    }

    async fn update(&self, conflict: &Conflict) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        conn.execute(
            r#"
            UPDATE conflicts SET
                status = ?2, resolved_at = ?3
            WHERE id = ?1
            "#,
            params![
                conflict.id.to_string(),
                conflict.status.to_string(),
                conflict.resolved_at.map(|dt| dt.to_rfc3339()),
            ],
        )?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;
        conn.execute("DELETE FROM conflicts WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Conflict>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let mut stmt = conn.prepare("SELECT * FROM conflicts ORDER BY detected_at DESC")?;
        let conflicts = stmt
            .query_map([], Self::row_to_conflict)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(conflicts)
    }

    async fn list_unresolved(&self) -> Result<Vec<Conflict>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT * FROM conflicts WHERE status = 'unresolved' ORDER BY detected_at DESC",
        )?;
        let conflicts = stmt
            .query_map([], Self::row_to_conflict)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(conflicts)
    }

    async fn list_by_skill(&self, skill_id: Uuid) -> Result<Vec<Conflict>> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;
        let skill_id_str = skill_id.to_string();

        let mut stmt = conn.prepare(
            "SELECT * FROM conflicts WHERE skill_a_id = ?1 OR skill_b_id = ?1 ORDER BY detected_at DESC",
        )?;
        let conflicts = stmt
            .query_map(params![skill_id_str], Self::row_to_conflict)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(conflicts)
    }

    async fn delete_by_skill(&self, skill_id: Uuid) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| Error::database(e.to_string()))?;
        let skill_id_str = skill_id.to_string();

        conn.execute(
            "DELETE FROM conflicts WHERE skill_a_id = ?1 OR skill_b_id = ?1",
            params![skill_id_str],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SkillSource;

    #[tokio::test]
    async fn test_skill_repository_crud() {
        let repo = SqliteSkillRepository::in_memory().unwrap();

        let skill = Skill::new("test-skill", SkillSource::Inline, SkillScope::Global);

        // Create
        repo.create(&skill).await.unwrap();

        // Read
        let retrieved = repo.get(skill.id).await.unwrap().unwrap();
        assert_eq!(retrieved.name, "test-skill");

        // Update
        let mut updated = retrieved.clone();
        updated.enabled = false;
        repo.update(&updated).await.unwrap();

        let after_update = repo.get(skill.id).await.unwrap().unwrap();
        assert!(!after_update.enabled);

        // Delete
        repo.delete(skill.id).await.unwrap();
        assert!(repo.get(skill.id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_skill_repository_list() {
        let repo = SqliteSkillRepository::in_memory().unwrap();

        let skill1 = Skill::builder("skill-1").priority(100).build();
        let skill2 = Skill::builder("skill-2").priority(50).build();

        repo.create(&skill1).await.unwrap();
        repo.create(&skill2).await.unwrap();

        let list = repo.list().await.unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "skill-1"); // Higher priority first
    }

    #[tokio::test]
    async fn test_skill_repository_search() {
        let repo = SqliteSkillRepository::in_memory().unwrap();

        let skill = Skill::builder("typescript-best")
            .description("TypeScript best practices")
            .build();
        repo.create(&skill).await.unwrap();

        let results = repo.search("typescript").await.unwrap();
        assert_eq!(results.len(), 1);

        let results = repo.search("python").await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_skill_repository_exists() {
        let repo = SqliteSkillRepository::in_memory().unwrap();

        let skill = Skill::new("my-skill", SkillSource::Inline, SkillScope::Global);
        repo.create(&skill).await.unwrap();

        assert!(repo.exists("my-skill").await.unwrap());
        assert!(!repo.exists("other-skill").await.unwrap());
    }
}
