use prost::Message;
use uuid::Uuid;

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/transaction.proto.rs"));
}

pub fn create() -> protos::Transaction {
    let mut transaction = protos::Transaction::default();
    transaction.id = Uuid::new_v4().as_bytes().to_vec();
    transaction.description = String::from("test");
    transaction.tags = vec![String::from("Entertainment"), String::from("Games")];
    transaction
}

pub fn serialize_transaction(transaction: &protos::Transaction) -> Vec<u8> {
    let mut buf = Vec::with_capacity(transaction.encoded_len());
    //TODO add error handling
    transaction.encode(&mut buf).unwrap();
    let mut size = (transaction.encoded_len() as u64).to_le_bytes().to_vec(); 
    size.extend_from_slice(&buf);
    size
}

#[cfg(test)]
mod tests {
    use std::any::type_name;
    use super::*;

    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn create_transaction_with_defaults() {
        let transaction = create();
        println!("{}", type_of(&transaction.id));
        assert_eq!(type_of(&transaction.id), "alloc::vec::Vec<u8>");
    }

    #[test]
    fn verify_serialize_transaction_with_length() {
        let transaction = create();
        let buf = serialize_transaction(&transaction);
        let expected_length = prost::length_delimiter_len(transaction.encoded_len()) + transaction.encoded_len();
        assert_eq!(expected_length, buf.len())
    }
}