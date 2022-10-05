use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Accounts {
    pub accounts: Arc<RwLock<HashMap<AccountId, Account>>>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(pub i32);

/** Implements the new function for creating and stanciating `Storage`s. */
impl Accounts {
    pub fn new() -> Self {
        Accounts {
            accounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn get_accounts() -> Accounts {
    let accounts = Accounts::new();
    accounts
}
