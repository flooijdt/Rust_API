use crate::error::Error;
use argon2::{self, Config};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::http::StatusCode;
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
pub struct AccountId(pub usize);

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
    /* Returns Error if it finds same email. */
    if storage.accounts.read().await.len() == 0 {
        account.id = Some(AccountId(1));
        storage
            .accounts
            .write()
            .await
            .insert(AccountId(1), account.clone());
        // .expect("Could not insert Account into storage1.");
        Ok(warp::reply::with_status("Account added.1", StatusCode::OK))
    } else {
        for mut acc in storage.accounts.read().await.clone().iter_mut() {
            if acc.1.email == account.email {
                println!("pelo menos neh");
                println!("{:#?}", storage.accounts.read().await.len());
                return Err(warp::reject::custom(Error::AccountAlreadyExist));
            } else {
                println!("Last else statement.");
                let acc_id = storage.accounts.read().await.len() + 1;
                account.id = Some(AccountId(acc_id));
                println!("Last else statement.2");
                println!("{:#?}", storage.accounts.read().await[&AccountId(1)]);
                println!("{:#?}", &account);
                storage
                    .accounts
                    .write()
                    .await
                    .insert(AccountId(acc_id), account.clone());
                // .expect("Could not insert Account into storage2.");
                println!("Last else statement.3");
                // .expect("Could not insert Account into storage2.");
                println!("iterating");
            }
            // return Ok(warp::reply::with_status("Account added.2", StatusCode::OK));
        }

        //commentareeo
        Ok(warp::reply::with_status("Account added.2", StatusCode::OK))
        // acc.0 = &AccountId(acc_counter).clone();
    }
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).expect("Could not hash password.")
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password,
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        ),
        Err(e) => Err(warp::reject::custom(e)),
    }
}