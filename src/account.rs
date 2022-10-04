#[derive(Debug, Clone)]
pub struct Accounts {
    pub clients: Arc<RwLock<HashMap<ClientId, Client>>>,
}
/** Implements the new function for creating and stanciating `Storage`s. */
impl Storage {
    pub fn new() -> Self {
        Storage {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
