use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum EnvArgsParseError {
    ArgumentsTooShort,
    ExpectedCsvFile { passed: String },
}

pub fn env_args_parse_file(args: Vec<String>) -> Result<String, EnvArgsParseError> {
    const MIN_ARG_LEN: usize = 2;
    const FILE_ARG: usize = 1;
    if args.len() < MIN_ARG_LEN {
        return Err(EnvArgsParseError::ArgumentsTooShort);
    }

    let file_arg = args[FILE_ARG].clone();
    let file_path = Path::new(&args[FILE_ARG]);

    let invalid_file_error = EnvArgsParseError::ExpectedCsvFile {
        passed: file_arg.clone(),
    };

    match file_path.extension() {
        Some(ext) => {
            if ext != "csv" {
                return Err(invalid_file_error);
            }
        }
        None => return Err(invalid_file_error),
    }

    Ok(file_arg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_args_parse_file_args_too_short_returns_err_args_to_short() {
        let args = vec![];
        let actual = env_args_parse_file(args);
        assert_eq!(true, actual.is_err());
        assert_eq!(EnvArgsParseError::ArgumentsTooShort, actual.unwrap_err());
    }

    #[test]
    fn env_args_parse_file_no_extension_returns_err_not_csv() {
        let test_file = "transactions";
        let args = vec!["target\\debug\\payments.exe", test_file]
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let actual = env_args_parse_file(args);
        assert_eq!(true, actual.is_err());

        let expected = EnvArgsParseError::ExpectedCsvFile {
            passed: String::from(test_file),
        };

        assert_eq!(expected, actual.unwrap_err());
    }

    #[test]
    fn env_args_parse_file_not_csv_returns_err_not_csv() {
        let test_files = vec!["transactions.csvs", ".css", " ", "blah", "foo.bar", ".csv"];
        for test_file in test_files {
            let args = vec!["target\\debug\\payments.exe", test_file]
                .iter()
                .map(|s| String::from(*s))
                .collect();

            let actual = env_args_parse_file(args);
            assert_eq!(true, actual.is_err());

            let expected = EnvArgsParseError::ExpectedCsvFile {
                passed: String::from(test_file),
            };

            assert_eq!(expected, actual.unwrap_err());
        }
    }

    #[test]
    fn env_args_parse_file_valid_csv_returns_ok_path() {
        let test_files = vec!["transactions.csv", "c::/derp.csv"];

        for test_file in test_files {
            let args = vec!["target\\debug\\payments.exe", test_file]
                .iter()
                .map(|s| String::from(*s))
                .collect();

            let actual = env_args_parse_file(args);
            assert_eq!(true, actual.is_ok());

            assert_eq!(String::from(test_file), actual.unwrap());
        }
    }
}
