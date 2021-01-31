use crate::{amount::Amount, client::ClientId};

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
    /// Returns the amount for the given transaction
    pub fn amount(&self) -> Option<Amount> {
        match self.transaction_type {
            TransactionType::Deposit(amount) => Some(amount),
            TransactionType::Withdrawal(amount) => Some(amount),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transaction(transaction_type: TransactionType) -> Transaction {
        Transaction {
            transaction_type,
            client: 0,
            id: 1,
        }
    }

    #[test]
    fn transaction_amount_returns_amount() {
        let amount = Amount::new(1);
        assert_eq!(
            Some(amount),
            transaction(TransactionType::Deposit(amount)).amount()
        );

        let amount = Amount::new(2231);
        assert_eq!(
            Some(amount),
            transaction(TransactionType::Withdrawal(amount)).amount()
        );

        let types_without_amounts = vec![
            TransactionType::Dispute,
            TransactionType::Resolve,
            TransactionType::Chargeback,
        ];

        for t in types_without_amounts {
            assert_eq!(None, transaction(t).amount());
        }
    }
}
