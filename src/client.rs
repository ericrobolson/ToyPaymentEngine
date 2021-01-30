use crate::amount::Amount;
use crate::transaction::{Transaction, TransactionId, TransactionType};

pub type ClientId = u16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionState {
    Ok,
    Disputed,
    Resolved,
    Chargebacked,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Client {
    id: ClientId,
    available: Amount,
    held: Amount,
    locked: bool,
    transactions: Vec<(TransactionState, Transaction)>,
}
impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            available: Amount::zero(),
            held: Amount::zero(),
            locked: false,
            transactions: vec![],
        }
    }

    pub fn id(&self) -> ClientId {
        self.id
    }
    pub fn available(&self) -> Amount {
        self.available
    }
    pub fn held(&self) -> Amount {
        self.held
    }
    pub fn locked(&self) -> bool {
        self.locked
    }
    pub fn total(&self) -> Amount {
        self.available() + self.held()
    }

    fn transaction_index(&self, transaction_id: TransactionId) -> Option<usize> {
        None
    }

    pub fn execute_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), TransactionError> {
        // Only apply if it matches this client
        if transaction.client != self.id {
            return Err(TransactionError::InvalidClient {
                actual: transaction.client,
                expected: self.id,
            });
        }

        // Check if frozen
        if self.locked {
            return Err(TransactionError::ClientLocked);
        }

        // Attempt to apply the transaction
        match transaction.transaction_type {
            crate::transaction::TransactionType::Deposit(amount) => {
                if amount.less_than_zero() {
                    return Err(TransactionError::InvalidDeposit { amount });
                }

                self.available = self.available + amount;
            }
            crate::transaction::TransactionType::Withdrawal(amount) => {
                let diff = self.available - amount;

                if amount.less_than_zero() || diff.less_than_zero() {
                    return Err(TransactionError::InvalidWithdrawal {
                        resulting_amount: diff,
                    });
                }

                self.available = diff;
            }
            crate::transaction::TransactionType::Dispute => {
                match self.transaction_index(transaction.id) {
                    Some(transaction_index) => {
                        todo!("IMPLEMENTE THIS!");
                        // TODO: ensure that only things in a valid state are processed
                    }
                    None => {
                        return Err(TransactionError::NotFound {
                            transaction_id: transaction.id,
                        });
                    }
                }
            }
            crate::transaction::TransactionType::Resolve => {
                match self.transaction_index(transaction.id) {
                    Some(transaction_index) => {
                        todo!("IMPLEMENTE THIS!");
                        // TODO: ensure that only things in a valid state are processed
                    }
                    None => {
                        return Err(TransactionError::NotFound {
                            transaction_id: transaction.id,
                        });
                    }
                }
            }
            crate::transaction::TransactionType::Chargeback => {
                match self.transaction_index(transaction.id) {
                    Some(transaction_index) => {
                        todo!("IMPLEMENTE THIS!");
                        // TODO: ensure that only things in a valid state are processed
                    }
                    None => {
                        return Err(TransactionError::NotFound {
                            transaction_id: transaction.id,
                        });
                    }
                }
            }
        }

        // It was a valid transaction, so log it
        self.transactions.push((TransactionState::Ok, transaction));

        Ok(())
    }
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
    AlreadyProcessed {
        current_state: TransactionState,
    },
    ClientLocked,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_transaction(
        client: &Client,
        transaction_id: TransactionId,
        transaction_type: TransactionType,
    ) -> Transaction {
        Transaction {
            client: client.id,
            id: transaction_id,
            transaction_type,
        }
    }

    fn create_deposit(client: &Client, amount: Amount) -> Transaction {
        create_transaction(client, 23, TransactionType::Deposit(amount))
    }

    fn create_withdrawal(client: &Client, amount: Amount) -> Transaction {
        create_transaction(client, 24, TransactionType::Withdrawal(amount))
    }

    fn create_dispute(client: &Client, id: TransactionId) -> Transaction {
        create_transaction(client, id, TransactionType::Dispute)
    }

    fn create_resolve(client: &Client, id: TransactionId) -> Transaction {
        create_transaction(client, id, TransactionType::Resolve)
    }

    fn create_chargeback(client: &Client, id: TransactionId) -> Transaction {
        create_transaction(client, id, TransactionType::Chargeback)
    }

    #[test]
    fn client_execute_transaction_dispute_transaction_doesnt_exist_does_nothing() {
        let mut client = Client::new(4482);

        let deposit = create_deposit(&client, Amount::new(40000));
        client.execute_transaction(deposit).unwrap();
        let withdrawal = create_withdrawal(&client, Amount::new(40000));
        client.execute_transaction(withdrawal).unwrap();

        let dispute = create_dispute(&client, 29292);
        let result = client.execute_transaction(dispute);
        assert_eq!(true, result.is_err());
        assert_eq!(
            TransactionError::NotFound {
                transaction_id: dispute.id
            },
            result.unwrap_err()
        );

        assert_eq!(2, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_dispute_is_locked_returns_err() {
        let mut client = Client::new(4482);

        client.locked = true;

        let transaction = create_dispute(&client, 29292);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        assert_eq!(TransactionError::ClientLocked, result.unwrap_err());

        assert_eq!(0, client.transactions.len());
    }

    // RESOLVES

    #[test]
    fn client_execute_transaction_resolve_transaction_doesnt_exist_does_nothing() {
        let mut client = Client::new(4482);

        let deposit = create_deposit(&client, Amount::new(40000));
        client.execute_transaction(deposit).unwrap();
        let withdrawal = create_withdrawal(&client, Amount::new(40000));
        client.execute_transaction(withdrawal).unwrap();

        let resolve = create_resolve(&client, 29292);
        let result = client.execute_transaction(resolve);
        assert_eq!(true, result.is_err());
        assert_eq!(
            TransactionError::NotFound {
                transaction_id: resolve.id
            },
            result.unwrap_err()
        );

        assert_eq!(2, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_resolve_is_locked_returns_err() {
        let mut client = Client::new(4482);

        client.locked = true;

        let transaction = create_resolve(&client, 29292);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        assert_eq!(TransactionError::ClientLocked, result.unwrap_err());

        assert_eq!(0, client.transactions.len());
    }

    //CHARGEBACKS
    #[test]
    fn client_execute_transaction_chargeback_transaction_doesnt_exist_does_nothing() {
        let mut client = Client::new(4482);

        let deposit = create_deposit(&client, Amount::new(40000));
        client.execute_transaction(deposit).unwrap();
        let withdrawal = create_withdrawal(&client, Amount::new(40000));
        client.execute_transaction(withdrawal).unwrap();

        let chargeback = create_chargeback(&client, 29292);
        let result = client.execute_transaction(chargeback);
        assert_eq!(true, result.is_err());
        assert_eq!(
            TransactionError::NotFound {
                transaction_id: chargeback.id
            },
            result.unwrap_err()
        );

        assert_eq!(2, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_chargeback_is_locked_returns_err() {
        let mut client = Client::new(4482);

        client.locked = true;

        let transaction = create_chargeback(&client, 29292);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        assert_eq!(TransactionError::ClientLocked, result.unwrap_err());

        assert_eq!(0, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_withdrawal_negative_returns_err() {
        let mut client = Client::new(4482);

        let transaction = create_deposit(&client, Amount::new(40000));
        client.execute_transaction(transaction).unwrap();

        let amount = Amount::new(-1);
        let transaction = create_withdrawal(&client, amount);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        let result = result.unwrap_err();
        let expected = TransactionError::InvalidWithdrawal {
            resulting_amount: client.available - amount,
        };

        assert_eq!(expected, result);
        assert_eq!(1, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_withdrawal_would_be_negative_returns_err() {
        let mut client = Client::new(4482);

        let transaction = create_deposit(&client, Amount::new(40000));
        client.execute_transaction(transaction).unwrap();

        let amount = Amount::new(40001);
        let transaction = create_withdrawal(&client, amount);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        let result = result.unwrap_err();
        let expected = TransactionError::InvalidWithdrawal {
            resulting_amount: client.available - amount,
        };

        assert_eq!(expected, result);
        assert_eq!(1, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_withdrawal_zero_returns_ok() {
        let mut client = Client::new(4482);

        let original_amount = Amount::new(40000);
        let transaction = create_deposit(&client, original_amount);
        client.execute_transaction(transaction).unwrap();

        let amount = Amount::new(0);
        let transaction = create_withdrawal(&client, amount);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_ok());
        assert_eq!(original_amount - amount, client.available);

        assert_eq!(2, client.transactions.len());
        assert_eq!((TransactionState::Ok, transaction), client.transactions[1]);
    }

    #[test]
    fn client_execute_transaction_withdrawal_valid_returns_ok() {
        let mut client = Client::new(4482);

        let original_amount = Amount::new(40000);
        let transaction = create_deposit(&client, original_amount);
        client.execute_transaction(transaction).unwrap();

        let amount = Amount::new(1);
        let transaction = create_withdrawal(&client, amount);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_ok());
        assert_eq!(original_amount - amount, client.available);

        assert_eq!(2, client.transactions.len());
        assert_eq!((TransactionState::Ok, transaction), client.transactions[1]);
    }

    #[test]
    fn client_execute_transaction_withdrawal_is_locked_returns_err() {
        let mut client = Client::new(4482);

        client.locked = true;

        let amount = Amount::new(1);
        let transaction = create_withdrawal(&client, amount);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        assert_eq!(TransactionError::ClientLocked, result.unwrap_err());

        assert_eq!(0, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_deposit_negative_returns_err() {
        let mut client = Client::new(4482);
        let deposit_amount = Amount::new(-1);

        let transaction = create_deposit(&client, deposit_amount);

        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());

        let error = result.unwrap_err();
        let expected = TransactionError::InvalidDeposit {
            amount: deposit_amount,
        };

        assert_eq!(expected, error);
        assert_eq!(0, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_deposit_zero_returns_ok() {
        let mut client = Client::new(4482);
        let deposit_amount = Amount::new(0);
        let transaction = create_deposit(&client, deposit_amount);

        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_ok());

        assert_eq!(Amount::zero(), client.available);
        assert_eq!((TransactionState::Ok, transaction), client.transactions[0]);
    }

    #[test]
    fn client_execute_transaction_deposit_valid_returns_ok() {
        let mut client = Client::new(4482);
        let deposit_amount = Amount::new(10120);
        let transaction = create_deposit(&client, deposit_amount);

        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_ok());

        assert_eq!(deposit_amount, client.available);
        assert_eq!((TransactionState::Ok, transaction), client.transactions[0]);
    }

    #[test]
    fn client_execute_transaction_deposit_is_locked_returns_err() {
        let mut client = Client::new(4482);

        client.locked = true;

        let amount = Amount::new(1);
        let transaction = create_deposit(&client, amount);
        let result = client.execute_transaction(transaction);

        assert_eq!(true, result.is_err());
        assert_eq!(TransactionError::ClientLocked, result.unwrap_err());

        assert_eq!(0, client.transactions.len());
    }

    #[test]
    fn client_execute_transaction_mismatched_client_returns_err() {
        let mut client = Client::new(4482);
        let transaction = Transaction {
            client: 25,
            id: 23,
            transaction_type: TransactionType::Resolve,
        };

        let result = client.execute_transaction(transaction);
        assert_eq!(true, result.is_err());

        let error = result.unwrap_err();
        let expected = TransactionError::InvalidClient {
            expected: client.id(),
            actual: transaction.client,
        };

        assert_eq!(expected, error);
        assert_eq!(0, client.transactions.len());
    }

    #[test]
    fn client_total_returns_expected() {
        let held = Amount::new(428382);
        let available = Amount::new(1);

        let mut client = Client::new(314);
        client.held = held;
        client.available = available;

        let expected = held + available;
        let actual = client.total();

        assert_eq!(expected, actual);
    }

    #[test]
    fn client_locked_returns_expected() {
        let mut client = Client::new(314);
        client.locked = true;

        assert_eq!(true, client.locked());
    }

    #[test]
    fn client_held_returns_expected() {
        let held = Amount::new(428382);
        let mut client = Client::new(314);

        client.held = held;
        assert_eq!(held, client.held());
    }

    #[test]
    fn client_available_returns_expected() {
        let available = Amount::new(48382);
        let mut client = Client::new(314);

        client.available = available;
        assert_eq!(available, client.available());
    }

    #[test]
    fn client_id_returns_expected() {
        let id: ClientId = 124;

        let actual = Client::new(id).id();
        let expected = id;

        assert_eq!(expected, actual);
    }

    #[test]
    fn client_new_returns_expected() {
        let id: ClientId = 124;

        let actual = Client::new(id);
        let expected = Client {
            id,
            available: Amount::zero(),
            held: Amount::zero(),
            locked: false,
            transactions: vec![],
        };

        assert_eq!(expected, actual);
    }
}
