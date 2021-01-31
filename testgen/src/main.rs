use rand::{seq::SliceRandom, Rng};

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

type ClientId = u16;
type TransactionId = u32;

const MAX_TRANSACTIONS: TransactionId = 50000;
const MAX_CLIENT_ID: ClientId = 10;

fn main() {
    println!("Generating test file");
    let mut rng = rand::thread_rng();

    let file = File::create("test.csv").unwrap();
    let mut file = LineWriter::new(file);

    let mut contents = vec![];

    for byte in "type, client, tx, amount\r\n".as_bytes() {
        contents.push(*byte);
    }

    let mut transactions: Vec<TransactionId> = (0..MAX_TRANSACTIONS).collect();
    transactions.shuffle(&mut rng);

    for transaction in transactions {
        let client_id: ClientId = rng.gen_range(0..MAX_CLIENT_ID);

        for byte in gen_line(
            client_id,
            transaction,
            gen_random_type().to_string(),
            gen_random_amount(),
        )
        .replace("\"", "")
        .as_bytes()
        {
            contents.push(*byte);
        }
    }

    file.write_all(&contents).unwrap();

    file.flush().unwrap();
}

fn gen_random_amount() -> String {
    let mut rng = rand::thread_rng();
    let rng_val: f32 = rng.gen();
    format!("{:?}", rng_val)
}

fn gen_random_type() -> &'static str {
    let mut rng = rand::thread_rng();
    let rng_val: u8 = rng.gen();
    let rng_val = rng_val % 6;
    match rng_val {
        0 => "deposit",
        1 => "withdrawal",
        2 => "dispute",
        3 => "resolve",
        4 => "chargeback",
        _ => "deposit",
    }
}

fn gen_line(
    client: ClientId,
    transaction: TransactionId,
    transaction_type: String,
    amount: String,
) -> String {
    let mut rng = rand::thread_rng();
    let rng_val: u8 = rng.gen();

    // Randomly include an amount
    if rng_val > u8::MAX / 2 {
        let newline = if rng_val > u8::MAX / 2 + u8::MAX / 4 {
            "\n"
        } else {
            "\r\n"
        };

        format!(
            "{:?}, {:?}, {:?}, {:?}\n",
            transaction_type, client, &transaction, &amount
        )
    } else {
        format!("{:?}, {:?}, {:?}\n", transaction_type, client, &transaction)
    }
}
