use std::env;

mod amount;
mod parse_csv;
mod parse_env_args;
mod transaction;
use parse_env_args::{env_args_parse_file, EnvArgsParseError};

#[derive(PartialEq, Debug)]
pub enum ApplicationError {
    EnvArgs(EnvArgsParseError),
}

fn main() -> Result<(), ApplicationError> {
    let args: Vec<String> = env::args().collect();

    let file_path = match env_args_parse_file(args) {
        Ok(path) => path,
        Err(e) => {
            return Err(ApplicationError::EnvArgs(e));
        }
    };

    // TODO: process
    // TODO: output

    let mut database = transaction::Database::new();

    for line in database.output() {
        println!("{:?}", line);
    }

    Ok(())
}
