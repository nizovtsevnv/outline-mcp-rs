//! HTTP session management
//!
//! Manages MCP sessions with UUID-based identifiers and automatic expiration.

use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;

/// MCP session data
#[derive(Debug)]
pub struct Session {
    /// Unique session identifier
    #[allow(dead_code)]
    pub id: String,
    /// When the session was created
    #[allow(dead_code)]
    pub created_at: Instant,
    /// Last time the session was accessed
    pub last_access: Instant,
}

/// Thread-safe session manager
#[derive(Debug)]
pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
    timeout_secs: u64,
}

impl SessionManager {
    /// Create a new session manager with the given timeout in seconds
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            timeout_secs,
        }
    }

    /// Create a new session and return its ID
    pub async fn create(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Instant::now();
        let session = Session {
            id: id.clone(),
            created_at: now,
            last_access: now,
        };
        self.sessions.write().await.insert(id.clone(), session);
        id
    }

    /// Check if a session exists and update its last access time
    pub async fn touch(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_access = Instant::now();
            true
        } else {
            false
        }
    }

    /// Check if a session exists without updating access time
    #[allow(dead_code)]
    pub async fn exists(&self, session_id: &str) -> bool {
        self.sessions.read().await.contains_key(session_id)
    }

    /// Remove a session
    pub async fn remove(&self, session_id: &str) -> bool {
        self.sessions.write().await.remove(session_id).is_some()
    }

    /// Remove expired sessions, returns the number of sessions removed
    pub async fn cleanup_expired(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let before = sessions.len();
        let timeout = std::time::Duration::from_secs(self.timeout_secs);
        sessions.retain(|_, session| session.last_access.elapsed() < timeout);
        before - sessions.len()
    }

    /// Get the current number of active sessions
    #[allow(dead_code)]
    pub async fn count(&self) -> usize {
        self.sessions.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_session() {
        let manager = SessionManager::new(1800);
        let id = manager.create().await;
        assert!(!id.is_empty());
        assert!(manager.exists(&id).await);
    }

    #[tokio::test]
    async fn test_touch_session() {
        let manager = SessionManager::new(1800);
        let id = manager.create().await;
        assert!(manager.touch(&id).await);
        assert!(!manager.touch("nonexistent").await);
    }

    #[tokio::test]
    async fn test_remove_session() {
        let manager = SessionManager::new(1800);
        let id = manager.create().await;
        assert!(manager.remove(&id).await);
        assert!(!manager.exists(&id).await);
        assert!(!manager.remove(&id).await);
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let manager = SessionManager::new(0); // 0 second timeout
        let _id = manager.create().await;
        // Session should expire immediately
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let removed = manager.cleanup_expired().await;
        assert_eq!(removed, 1);
        assert_eq!(manager.count().await, 0);
    }

    #[tokio::test]
    async fn test_session_count() {
        let manager = SessionManager::new(1800);
        assert_eq!(manager.count().await, 0);
        manager.create().await;
        manager.create().await;
        assert_eq!(manager.count().await, 2);
    }
}
