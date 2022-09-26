use crate::client::{Client};

pub struct GetResponse {
    pub pageNumber: usize,
    pub pageSize: usize,
    pub totalCount: usize,
    pub clients: Vec<Client>,
  }
