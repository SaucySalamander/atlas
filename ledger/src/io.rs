use std::{
    fmt::Error,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, Write, ErrorKind},
    path::{Path, PathBuf},
};

use prost::Message;

use crate::transaction::protos::{self, Transaction};

const DEFAULT_LEDGER_PATH: &str = "./data/ledger.data";

pub fn create_ledger() -> Result<PathBuf, Error> {
    if !Path::new(DEFAULT_LEDGER_PATH).exists() {
        let (directory, _) = DEFAULT_LEDGER_PATH.rsplit_once("/").unwrap();
        let result = fs::create_dir_all(directory);
        match result {
            Err(e) => panic!("Failed to create file {}", e),
            Ok(()) => create_file(DEFAULT_LEDGER_PATH),
        }
    }

    Ok(PathBuf::from(DEFAULT_LEDGER_PATH))
}

fn create_file(path: &str) {
    let result = File::create(path);
    match result {
        Err(e) => panic!("Failed to create file {}", e),
        Ok(_) => (),
    }
}

pub fn write_record(bytes: &Vec<u8>) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(DEFAULT_LEDGER_PATH);

    let mut file = match file {
        Err(e) => panic!("Could not open file {}", e),
        Ok(f) => f,
    };

    write_bytes(&mut file, bytes);
}

fn write_bytes(file: &mut File, bytes: &Vec<u8>) {
    let result = file.write_all(&bytes);
    match result {
        Err(e) => panic!("Failed to write bytes {}", e),
        Ok(()) => file.flush().unwrap(),
    }
}

pub fn read_all_records() -> Vec<Transaction> {
    let file = File::open(DEFAULT_LEDGER_PATH);

    let mut file = match file {
        Err(e) => panic!("Failed to open file. {}", e),
        Ok(f) => f,
    };

    let mut transactions: Vec<Transaction> = Vec::new();

    loop {
        let pos = file.stream_position().unwrap();

        let mut size_bytes = u64::to_le_bytes(0).to_vec();
        let read_result = file.read_exact(&mut size_bytes);
        match read_result {
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => panic!("unkown error {}", e),
            },
            Ok(()) => ()
        };
        
        let offset = size_bytes.len() - 1;

        let size = u64::from_le_bytes(size_bytes.try_into().unwrap());
        let mut buf: Vec<u8> = vec![0 as u8; size.try_into().unwrap()];
        file.read_exact(&mut buf).unwrap();

        let result = protos::Transaction::decode(buf.as_slice());
        let transaction = match result {
            Err(e) => panic!("Failed to decode message {}", e),
            Ok(t) => t,
        };

        transactions.push(transaction);
    }
    transactions
}

pub fn read_one_record() {}

pub fn update_ledger() {}

#[cfg(test)]
mod tests {
    use super::create_ledger;

    #[test]
    pub fn test_create_ledger_file_is_not_exist() {
        let ledger_file = create_ledger();
        match ledger_file {
            Err(e) => println!("Failed test"),
            Ok(path) => println!("good test"),
        }
    }
}
