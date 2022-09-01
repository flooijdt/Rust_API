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

