//! Authentication and rate limiting
//!
//! Validates MCP access tokens and enforces per-IP rate limits using a token bucket algorithm.

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::time::Instant;
use tokio::sync::RwLock;

/// Rate limit bucket using token bucket algorithm
#[derive(Debug)]
struct RateBucket {
    tokens: f64,
    last_refill: Instant,
}

/// Authentication and rate limiting guard
#[derive(Debug)]
pub struct AuthGuard {
    allowed_tokens: HashSet<String>,
    rate_limits: RwLock<HashMap<IpAddr, RateBucket>>,
    requests_per_minute: u32,
}

impl AuthGuard {
    /// Create a new auth guard with allowed tokens and rate limit
    pub fn new(tokens: Vec<String>, requests_per_minute: u32) -> Self {
        Self {
            allowed_tokens: tokens.into_iter().collect(),
            rate_limits: RwLock::new(HashMap::new()),
            requests_per_minute,
        }
    }

    /// Validate an MCP access token from the X-MCP-Token header
    pub fn validate_token(&self, token: &str) -> bool {
        self.allowed_tokens.contains(token)
    }

    /// Check and consume a rate limit token for the given IP address.
    /// Returns `true` if the request is allowed, `false` if rate limited.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let max_tokens = f64::from(self.requests_per_minute);
        let refill_rate = max_tokens / 60.0;

        let mut limits = self.rate_limits.write().await;
        let bucket = limits.entry(ip).or_insert_with(|| RateBucket {
            tokens: max_tokens,
            last_refill: Instant::now(),
        });

        // Refill tokens based on elapsed time
        let elapsed = bucket.last_refill.elapsed().as_secs_f64();
        bucket.tokens = elapsed.mul_add(refill_rate, bucket.tokens).min(max_tokens);
        bucket.last_refill = Instant::now();

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Remove stale rate limit entries (IPs not seen for over 5 minutes)
    pub async fn cleanup_stale(&self) {
        let mut limits = self.rate_limits.write().await;
        let cutoff = std::time::Duration::from_secs(300);
        limits.retain(|_, bucket| bucket.last_refill.elapsed() < cutoff);
    }

    /// Extract MCP token from the X-MCP-Token header
    pub fn extract_mcp_token(req: &hyper::Request<hyper::body::Incoming>) -> Option<&str> {
        req.headers()
            .get("X-MCP-Token")
            .and_then(|v| v.to_str().ok())
    }

    /// Extract Outline API key from the Authorization header (Bearer token)
    pub fn extract_outline_key(req: &hyper::Request<hyper::body::Incoming>) -> Option<String> {
        req.headers()
            .get(hyper::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_token() {
        let guard = AuthGuard::new(vec!["token-abc".to_string()], 60);
        assert!(guard.validate_token("token-abc"));
        assert!(!guard.validate_token("wrong-token"));
    }

    #[tokio::test]
    async fn test_rate_limit_allows_requests() {
        let guard = AuthGuard::new(vec![], 60);
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        // First request should be allowed
        assert!(guard.check_rate_limit(ip).await);
    }

    #[tokio::test]
    async fn test_rate_limit_blocks_excess() {
        let guard = AuthGuard::new(vec![], 2); // 2 requests per minute
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        assert!(guard.check_rate_limit(ip).await);
        assert!(guard.check_rate_limit(ip).await);
        // Third request should be blocked
        assert!(!guard.check_rate_limit(ip).await);
    }

    #[tokio::test]
    async fn test_cleanup_stale() {
        let guard = AuthGuard::new(vec![], 60);
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        guard.check_rate_limit(ip).await;
        // Entry exists but is fresh, cleanup shouldn't remove it
        guard.cleanup_stale().await;
        // Still should be allowed
        assert!(guard.check_rate_limit(ip).await);
    }
}
