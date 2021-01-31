use crate::{
    amount::Amount,
    client::ClientId,
    transaction::{Transaction, TransactionId, TransactionType},
};
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn execute(file_path: String) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Note: I ran into an issue with whitespace, so just replaced it all to get it working.
    contents = contents.replace("\r\n", "\n").replace(" ", "");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .terminator(csv::Terminator::CRLF)
        .from_reader(contents.as_bytes());

    let mut transactions = vec![];

    for result in rdr.deserialize() {
        let record: CsvTransaction = result?;

        let transaction = record.into_transaction()?;
        match transaction {
            Some(transaction) => {
                transactions.push(transaction);
            }
            None => {}
        }
    }
    Ok(transactions)
}

#[derive(serde::Deserialize, Debug)]
pub struct CsvTransaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub client: String,
    pub tx: TransactionId,
    pub amount: Option<String>,
}

impl CsvTransaction {
    pub fn into_transaction(&self) -> Result<Option<Transaction>, Box<dyn Error>> {
        let amount = self.amount.clone().unwrap_or("".to_string());

        let amount_empty = amount.trim() == "";

        let transaction_type = match self.transaction_type.trim() {
            "deposit" => {
                // TODO: With more time, implement an actual parse error here. For now fail gracefully by ignoring.
                if amount_empty {
                    return Ok(None);
                }

                let amount = Amount::from_str(&amount)?;
                TransactionType::Deposit(amount)
            }
            "withdrawal" => {
                // TODO: With more time, implement an actual parse error here. For now fail gracefully by ignoring.
                if amount_empty {
                    return Ok(None);
                }

                let amount = Amount::from_str(&amount)?;
                TransactionType::Withdrawal(amount)
            }
            "dispute" => TransactionType::Dispute,
            "resolve" => TransactionType::Resolve,
            "chargeback" => TransactionType::Chargeback,
            _ => {
                // TODO: With more time, implement an actual parse error here.
                return Ok(None);
            }
        };

        let client_id = self.client.parse::<ClientId>()?;

        Ok(Some(Transaction {
            transaction_type,
            client: client_id,
            id: self.tx,
        }))
    }
}
