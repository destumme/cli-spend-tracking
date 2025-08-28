use sqlite::Connection;

pub struct TransactionRepository {
    connection: Connection,
    page_size: u32,
}

impl TransactionRepository {
    pub fn new(file_location: String, page_size: u32) -> Self {
        TransactionRepository {
            //todo: add errors
            connection: sqlite::open(file_location).expect("Cannot open sql connection"),
            page_size,
        }
    }

    pub fn load_inital_page() {
        let query = "SELECT * FROM spending.transactions LIMIT ?";
    }

    pub fn next_page() {
        let query = "SELECT * FROM spending.transactions WHERE uuid > ? and uuid <= ?";
    }
}
