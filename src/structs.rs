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
        // pub fn init() -> HashMap<ClientId, Client> {
        // let map: HashMap<ClientId, Client> = HashMap::new();
        // map
    }
    //     pub fn add_client(mut self, client: Client) -> Self {
    //     self.clients.write().insert(client.id.clone(), client);
    //     self
    // }
    // impl warp::Reply for Storage {
    //     fn into_response(self) -> warp::reply::Response {
    //         Response::new(format!("{}", self.json).into())
    //     }
    // }
    //

    #[derive(Debug, Clone)]
    pub struct Pagination {
        pub start: usize,
        pub end: usize,
    }

    /* Dealing with pagination errors */
    #[derive(Debug)]
    pub enum Error {
        ParseError(std::num::ParseIntError),
        MissingParameters,
        ClientNotFound,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
                Error::MissingParameters => write!(f, "Missing parameter"),
                Error::ClientNotFound => write!(f, "Client not found"),
            }
        }
    }

    impl Reject for Error {}

