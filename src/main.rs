use std::env;
use std::error::Error;

pub mod amount;
pub mod client;
mod database;
mod parse_csv;
mod parse_env_args;
pub mod transaction;
use parse_env_args::{env_args_parse_file, EnvArgsParseError};

#[derive(Debug)]
pub enum ApplicationError {
    EnvArgs(EnvArgsParseError),
    CsvParseError(Box<dyn Error>),
}

#[cfg(not(feature = "test-large-files"))]
fn main() -> Result<(), ApplicationError> {
    let args: Vec<String> = env::args().collect();

    let file_path = match env_args_parse_file(args) {
        Ok(path) => path,
        Err(e) => {
            return Err(ApplicationError::EnvArgs(e));
        }
    };

    let mut database = database::Database::new();

    let transactions = match parse_csv::execute(file_path) {
        Ok(transactions) => transactions,
        Err(e) => {
            return Err(ApplicationError::CsvParseError(e));
        }
    };

    for transaction in transactions {
        match database.apply_transaction(transaction) {
            Ok(_) => {
                // Succesfully processed, so no further actions.
            }
            Err(e) => {
                // TODO: error handling for invalid transactions?
            }
        }
    }

    database.output();

    Ok(())
}

// This is a simple way to test large files.
#[cfg(feature = "test-large-files")]
fn main() {
    test_large_files::execute();
}

#[cfg(feature = "test-large-files")]
mod test_large_files {
    use rand::{seq::SliceRandom, Rng};

    use crate::{
        amount::Amount,
        client::{Client, ClientAccount, ClientId},
        database::Database,
        transaction::{Transaction, TransactionError, TransactionId, TransactionType},
    };

    pub fn execute() {
        let mut rng = rand::thread_rng();

        let mut db = Database::<Client>::new();

        // Create some transactions
        let mut transactions = vec![];

        for transaction_id in 0..TransactionId::MAX as usize + 1 {
            if transaction_id % 100000 == 0 {
                println!(
                    "Build {:?} out of {:?} transactions. {:?}% complete.",
                    transaction_id,
                    TransactionId::MAX,
                    transaction_id / TransactionId::MAX as usize
                );
            }

            let client_id: ClientId = rng.gen();

            let transaction = Transaction {
                transaction_type: TransactionType::Deposit(Amount::new(342)),
                client: client_id,
                id: transaction_id as TransactionId,
            };

            transactions.push(transaction);
        }

        transactions.shuffle(&mut rng);

        for (i, transaction) in transactions.iter().enumerate() {
            if i % 100000 == 0 {
                println!(
                    "Build {:?} out of {:?} transactions. {:?}% complete.",
                    i,
                    TransactionId::MAX,
                    i / TransactionId::MAX as usize
                );
            }

            let db_result = db.apply_transaction(*transaction);
        }

        db.output();
    }
}
