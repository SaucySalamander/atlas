use std::path::PathBuf;

use transaction::protos::Transaction;

mod transaction;
mod io;

pub fn init() -> PathBuf {
    let result = io::create_ledger();
    match result {
        Err(_) =>  panic!("Failed to create ledger files") ,
        Ok(p) => return p,
    }
}

pub fn get_ledger() -> Vec<Transaction> {
    io::read_all_records()
}

pub fn create_transaction() {
    let transaction = transaction::create();
    let bytes = transaction::serialize_transaction(&transaction);
    io::write_record(&bytes);


}

pub fn update_ledger() {

}