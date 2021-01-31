use crate::{
    client::{Client, ClientAccount, ClientId},
    transaction::{Transaction, TransactionError, TransactionId, TransactionType},
};

#[derive(PartialEq, Debug)]
enum Status {
    Valid,
    Invalid,
}

pub struct Database<Account>
where
    Account: ClientAccount,
{
    clients: Vec<(Account, Status)>,
}

impl<Account> Database<Account>
where
    Account: ClientAccount,
{
    pub fn apply_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        let client_index = transaction.client as usize;

        self.clients[client_index].1 = Status::Valid;
        self.clients[client_index]
            .0
            .execute_transaction(transaction)
    }

    pub fn output(&self) {
        println!("client, available, held, total, locked");

        self.clients
            .iter()
            .filter(|(_account, status)| *status == Status::Valid)
            .map(|(account, _)| account)
            .for_each(|client| {
                println!(
                    "{:?}, {:?}, {:?}, {:?}, {:?}",
                    client.id(),
                    client.available(),
                    client.held(),
                    client.total(),
                    client.locked()
                );
            });
    }
}

impl Database<Client> {
    pub fn new() -> Self {
        let mut clients = Vec::with_capacity(ClientId::MAX as usize);

        for client_id in 0..ClientId::MAX as usize + 1 {
            clients.push((Client::new(client_id as ClientId), Status::Invalid));
        }
        Self { clients }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amount::Amount;

    #[test]
    fn database_new_returns_expected() {
        let db = Database::<Client>::new();
        for (id, (client, status)) in db.clients.iter().enumerate() {
            assert_eq!(id as ClientId, client.id());
            assert_eq!(Status::Invalid, *status);
        }
    }

    #[test]
    fn database_apply_transaction_sets_account_to_valid_returns_result() {
        // With more time, the ideal would have been to make a mock implementation of the ClientAccount trait and use it for testing.

        let mut db = Database::<Client>::new();
        let client_id = 45;

        let transaction = Transaction {
            transaction_type: TransactionType::Deposit(Amount::new(342)),
            client: client_id,
            id: 23,
        };
        let db_result = db.apply_transaction(transaction);

        assert_eq!(Status::Valid, db.clients[client_id as usize].1);
        assert_eq!(
            db.clients[client_id as usize]
                .0
                .execute_transaction(transaction),
            db_result
        );
    }

    #[test]
    fn database_apply_transaction_works_for_max_clients() {
        let mut db = Database::<Client>::new();

        for client in 0..ClientId::MAX as usize + 1 {
            let client_id = client as ClientId;
            let transaction = Transaction {
                transaction_type: TransactionType::Deposit(Amount::new(342)),
                client: client_id,
                id: 23,
            };
            let db_result = db.apply_transaction(transaction);

            assert_eq!(Status::Valid, db.clients[client_id as usize].1);
            assert_eq!(
                db.clients[client_id as usize]
                    .0
                    .execute_transaction(transaction),
                db_result
            );
        }
    }
}
