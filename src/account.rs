use crate::error::Error;
use argon2::{self, Config};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{http::StatusCode, Rejection};
/* Creates Accounts (storage) struct as Arc for async use. */
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
    Accounts::new()
}

/** Implements the POST function. */
pub async fn add_account(
    storage: Accounts,
    mut account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    /* Hash and salt password before storing it. */
    hash(account.password.as_bytes());
    /* Tries to get the Account via its Id. */
    // let account_given = storage
    //     .accounts
    //     .write()
    //     .await
    //     .get_mut(&account.id.clone().expect("Could not get requested id."))
    //     .cloned();
    let mut acc_counter = 1;
    /* Returns Error if it finds same email. */
    for mut acc in storage.accounts.read().await.clone().iter_mut() {
        if acc.1.email == account.email {
            return Err(warp::reject::custom(Error::AccountAlreadyExist));
        }
        acc_counter += 1;
        account.id = Some(AccountId(acc_counter).clone()).clone();
        storage
            .accounts
            .write()
            .await
            .insert(
                account
                    .clone()
                    .id
                    .expect("Could not insert id into storage."),
                account.clone(),
            )
            .expect("Could not insert Account into storage.");
        // return Ok(warp::reply::with_status("Account added.", StatusCode::OK));
        // return Ok(warp::reply::with_status("Account added.", StatusCode::OK));

        // acc.0 = &AccountId(acc_counter).clone();
    }

    Ok(warp::reply::with_status("Account added.", StatusCode::OK))
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).expect("Could not hash password.")
}
