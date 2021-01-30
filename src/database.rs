use crate::{
    client::{Client, ClientId},
    transaction::Transaction,
};

pub struct Database {
    clients: Vec<Client>,
}

impl Database {
    pub fn new() -> Self {
        let mut db = Self {
            clients: Vec::with_capacity(ClientId::MAX as usize),
        };

        // testing

        db
    }

    pub fn apply_transaction(&mut self, transaction: Transaction) {}

    pub fn output(&self) -> Vec<String> {
        //TODO: test this?
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
