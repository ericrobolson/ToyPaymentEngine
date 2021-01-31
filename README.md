# ToyPaymentEngine
A simple toy payment engine that process payment transactions from a CSV file.


## File Structure

The bulk of the work is done in `client.rs` as that's where an account may be modified through transactions.
There were quite a lot of edge cases that needed to be handled, so the best way to ensure they were done was
through test driven development. Enums were utilized heavily to denote states that needed to be handled.

Account amounts are defined in `amount.rs` which is a wrapper for `rust_decimal`, a decimal crate focused on 
the finance realm.

Transactions, their various forms, and their states are defined in `transaction.rs`. 

A database is defined in `database.rs` and is meant to keep track of all client accounts that are processed.
This is not heavy duty, as it resides in memory and is not asynchronous. 

CSV parsing is handled in `parse_csv.rs`.

A variety of test CSVs are located in the `/test` folder.


## Next Steps
* Look at `std` traits and see what can be applied here. Primarily the `std::convert::TryInto` ones.
* Better threading 
* Better error handling
* Handling truncation on `Amount`s when converting from a string