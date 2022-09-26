mod client;

pub struct GetResponse {
    pageNumber: u32,
    pageSize: u32,
    totalCount: u32,
    clients: Vec<Clients>,
  }
