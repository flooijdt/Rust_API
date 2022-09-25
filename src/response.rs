mod client;

pub struct Response {
    pageNumber: u32,
    pageSize: u32,
    totalCount: u32,
    clients: Vec<Clients>,
  }
