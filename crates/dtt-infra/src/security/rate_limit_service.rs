use dtt_core::ports::{RateLimiter, RateLimit};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryRateLimiter {
    limits: HashMap<String, RateLimit>,
    counters: Arc<Mutex<HashMap<String, u32>>>,
}

impl InMemoryRateLimiter {
    pub fn new() -> Self {
        let mut limits = HashMap::new();
        limits.insert("upload".to_string(), RateLimit { requests_per_minute: 5, burst_limit: 10 });
        limits.insert("feed".to_string(), RateLimit { requests_per_minute: 60, burst_limit: 100 });
        
        Self {
            limits,
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl RateLimiter<String> for InMemoryRateLimiter {
    async fn check_rate_limit(&self, user_id: &Uuid, action: &str) -> Result<bool, String> {
        let key = format!("{}:{}", user_id, action);
        let limit = self.limits.get(action).unwrap_or(&RateLimit { requests_per_minute: 30, burst_limit: 50 });
        
        let mut counters = self.counters.lock().unwrap();
        let current_count = counters.get(&key).unwrap_or(&0);
        
        Ok(*current_count < limit.burst_limit)
    }

    async fn record_request(&self, user_id: &Uuid, action: &str) -> Result<(), String> {
        let key = format!("{}:{}", user_id, action);
        let mut counters = self.counters.lock().unwrap();
        *counters.entry(key).or_insert(0) += 1;
        Ok(())
    }
}