use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;
use tokio::sync::RwLock;
use std::sync::Arc;
use warp::reject::Reject;

#[derive(Debug, Clone)]
pub struct Storage { pub clients: Arc<RwLock<HashMap<ClientId, Client>>>}

impl Storage {
    pub fn new() -> Self {
        Storage{ clients: Arc::new(RwLock::new(HashMap::new()))}
    }
}

