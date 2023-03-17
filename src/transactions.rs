pub fn import_transactions(filename: &String) {
    println!("TODO CREATE FUNCTION to read {}", filename);
}

pub fn list_all_transactions() {
    let ledger = ledger::get_ledger();

    for (index, transaction) in ledger.iter().enumerate() {
        println!("{}: {:?}", index, transaction);
    }
}