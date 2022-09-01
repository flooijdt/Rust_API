use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::client::{ClientId, Client};

#[derive(Debug, Clone)]
pub struct Storage { pub clients: Arc<RwLock<HashMap<ClientId, Client>>>}

impl Storage {
    pub fn new() -> Self {
        Storage{ clients: Arc::new(RwLock::new(HashMap::new()))}
    }
}

