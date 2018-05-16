extern crate time;
extern crate crypto;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;

use rustc_serialize::hex::ToHex;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn new_block(){
}

#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    amount: u32,
    recipient: String,
    sender: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    index: u32,
    previous_hash: String,
    proof: String,
    timestamp: u32,
    transactions: Vec<Transaction>
}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>
}

impl Blockchain {
    pub fn new_transaction(&mut self, transaction: Transaction) -> u32{
        self.current_transactions.push(transaction);
        let num = match self.chain.last()
            {
                Some(_) => self.chain.last().unwrap().index,
                None => 0
            };
        num+1
    }

    pub fn new_block(&mut self, proof: String, previous_hash: String) -> Block{
        let block = Block { index: self.chain.last().unwrap().index, previous_hash: previous_hash, proof: proof, timestamp: time::now().tm_nsec as u32,
            transactions: self.current_transactions.to_vec() };

        self.chain.push(block.clone());
        self.current_transactions = Vec::new();
        block
    }

    pub fn calculate_hash(self, input: &Block) -> String {
        let mut sha = Sha256::new();
        let json = serde_json::to_string(input).expect("Couldn't serialize block");
        sha.input_str(&json);
        sha.result_str()//.as_bytes().to_hex()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    #[test]
    fn new_transaction_is_added() {
        let vec: Vec<Block> = Vec::new();
        let trans: Vec<Transaction> = Vec::new();
        let mut chain = Blockchain { chain: vec, current_transactions: trans};

        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        assert_eq!(index, 1);
    }

    #[test]
    fn last_index_is_incremented() {
        let vec: Vec<Block> = Vec::new();
        let trans: Vec<Transaction> = Vec::new();
        let mut chain = Blockchain { chain: vec, current_transactions: trans};

        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        let index2 = chain.new_transaction(Transaction { amount: 10, recipient: "you".to_string(), sender: "me".to_string() });
        assert_eq!(index, 1);
        assert_eq!(2, chain.current_transactions.len());
        assert_eq!(index2, 2);
    }

    #[test]
    pub fn test_to_hex() {
        assert_eq!("foobar".as_bytes().to_hex(), "666f6f626172");
    }

}