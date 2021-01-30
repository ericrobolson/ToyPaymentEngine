use std::env;

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

    load_csv(file_path);
    // TODO: process
    // TODO: output

    Ok(())
}

fn load_csv(file_path: String) {
    unimplemented!();
}
