use uuid::Uuid;

#[derive(Debug, Default)]
pub struct TransactionRow {
    //v7 time based uuid
    uuid: Uuid,
    ignored: bool,
    account: String,
    date: String,
    summary: String,
    amount: u32,
    category: String,
    notes: String,
}

impl TransactionRow {}
