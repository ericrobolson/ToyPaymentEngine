pub type ClientId = u16;

pub type TransactionId = u32;

pub enum AmountError {
    SubtractOverflow,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Amount {
    whole_number: u32,
    decimal: [u8; 4],
}

impl Amount {
    // TODO: implement displays

    pub fn new() -> Self {
        Self {
            whole_number: 0,
            decimal: [0; 4],
        }
    }

    pub fn add(&self, other: Self) -> Self {
        *self
        // todo!();
    }

    pub fn subtract(&self, other: Self) -> Result<Self, AmountError> {
        Ok(*self)
        //todo!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TransactionType {
    Deposit(Amount),
    Withdrawl(Amount),
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: ClientId,
    pub id: TransactionId,
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

pub struct Client {
    id: ClientId,
    available: Amount,
    held: Amount,
    locked: bool,

    transactions: Vec<Transaction>,
}
impl Client {
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
        self.available().add(self.held())
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
        db.clients.push(Client {
            id: 01,
            available: Amount::new(),
            held: Amount::new(),
            locked: false,
            transactions: vec![],
        });

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
