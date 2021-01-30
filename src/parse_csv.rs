use crate::{
    amount::Amount,
    client::ClientId,
    transaction::{
        Transaction, TransactionError, TransactionId, TransactionState, TransactionType,
    },
};

pub struct CsvTransaction {
    pub transaction_type: String,
    pub client: ClientId,
    pub transaction_id: TransactionId,
    pub amount: Option<Amount>,
}

impl CsvTransaction {
    pub fn into_transaction(&self) -> Option<Transaction> {
        todo!();
    }
}
