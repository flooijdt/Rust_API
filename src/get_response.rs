use serde::{Deserialize, Serialize};
use std::slice;

use crate::client::Client;

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, Eq, PartialEq, Ord)]
pub struct GetResponse {
    pub pageNumber: usize,
    pub pageSize: usize,
    pub totalCount: usize,
    pub clients: Vec<Client>,
}

impl GetResponse {
    pub fn new() -> Self {
        GetResponse {
            pageNumber: 0,
            pageSize: 0,
            totalCount: 0,
            clients: Vec::new(),
        }
    }
}
