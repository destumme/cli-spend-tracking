use uuid::{Uuid, uuid};
#[derive(Debug, Default)]
pub struct TransactionTable {
    rows: Vec<super::row::TransactionRow>,
    start_cursor: Uuid,
    end_cursor: Uuid,
}

impl TransactionTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self) {}
}
