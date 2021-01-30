use crate::{
    amount::Amount,
    client::{Client, ClientId},
};

pub type TransactionId = u32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TransactionType {
    Deposit(Amount),
    Withdrawal(Amount),
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionState {
    Ok,
    Disputed,
    Chargebacked,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionError {
    InvalidClient {
        expected: ClientId,
        actual: ClientId,
    },
    InvalidDeposit {
        amount: Amount,
    },
    InvalidWithdrawal {
        resulting_amount: Amount,
    },
    NotFound {
        transaction_id: TransactionId,
    },
    Unprocessable {
        current_state: TransactionState,
        required_state: TransactionState,
    },
    ClientLocked,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: ClientId,
    pub id: TransactionId,
}

impl Transaction {
    // TODO: test
    pub fn amount(&self) -> Option<Amount> {
        match self.transaction_type {
            TransactionType::Deposit(amount) => Some(amount),
            TransactionType::Withdrawal(amount) => Some(amount),
            _ => None,
        }
    }
}

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
pub struct Database {
    clients: Vec<Client>,
}

impl Database {
    pub fn new() -> Self {
        let mut db = Self {
            clients: Vec::with_capacity(ClientId::MAX as usize),
        };

        // testing

        db
    }

    pub fn apply_transaction(&mut self, transaction: Transaction) {}

    pub fn output(&self) -> Vec<String> {
        let mut output = vec![];
        println!("client, available, held, total, locked");

        self.clients.iter().for_each(|client| {
            println!(
                "{:?}, {:?}, {:?}, {:?}, {:?}",
                client.id(),
                client.available(),
                client.held(),
                client.total(),
                client.locked()
            );
        });

        output
    }
}
