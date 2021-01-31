use std::env;
use std::error::Error;

mod amount;
mod client;
mod database;
mod parse_csv;
mod parse_env_args;
mod transaction;
use parse_env_args::{env_args_parse_file, EnvArgsParseError};

#[derive(Debug)]
pub enum ApplicationError {
    EnvArgs(EnvArgsParseError),
    CsvParseError(Box<dyn Error>),
}

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
                // TODO: error handling for invalid transactions.
            }
        }
    }

    database.output();

    Ok(())
}
