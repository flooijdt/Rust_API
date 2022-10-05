use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{http::StatusCode, Rejection};

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
/** Implements the POST function. */
pub async fn add_account(
    storage: Accounts,
    account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_given = storage
        .accounts
        .write()
        .await
        .get_mut(&account.id.clone().expect("Could not get requested id."))
        .cloned();

    match account_given {
        Some(_) => Err(warp::reject::custom(Error::AccountAlreadyExist)),
        None => {
            storage.accounts.write().await.insert(
                account.id.clone().expect("Could not insert id in storage."),
                account.clone(),
            );
            // .expect("Could not register account id and account.");
            Ok(warp::reply::with_status("Account added.", StatusCode::OK))
        }
    }
}
