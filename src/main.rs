use ledger;
mod transactions;
fn main() {
    transactions::import_transactions();

    ledger::init();
    let ledger = ledger::get_ledger();

    for (index, transaction) in ledger.iter().enumerate() {
        println!("{}: {:?}", index, transaction);
    }
}
