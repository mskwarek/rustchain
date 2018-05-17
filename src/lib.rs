extern crate time;
extern crate crypto;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;
extern crate uuid;

use rustc_serialize::hex::ToHex;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use uuid::Uuid;

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
    proof: u32,
    timestamp: u32,
    transactions: Vec<Transaction>
}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    node_identifier: String
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut vec: Vec<Block> = Vec::new();
        vec.push(Block { index: 1, previous_hash: "1".to_string(), proof: 100, timestamp: 333037375,
            transactions: Vec::new() });
        let my_uuid = Uuid::new_v4();
        Blockchain { chain: vec, current_transactions: Vec::new(), node_identifier: my_uuid.to_string().replace("-", "") }
    }
    pub fn new_transaction(&mut self, transaction: Transaction) -> u32 {
        self.current_transactions.push(transaction);
        let num = match self.chain.last()
            {
                Some(_) => self.chain.last().unwrap().index,
                None => 0
            };
        num + 1
    }

    pub fn new_block(&mut self, proof: u32, previous_hash: String) -> Block {
        let block = Block {
            index: self.chain.last().unwrap().index + 1,
            previous_hash: previous_hash,
            proof: proof,
            timestamp: time::now().tm_nsec as u32,
            transactions: self.current_transactions.to_vec()
        };

        self.chain.push(block.clone());
        self.current_transactions = Vec::new();
        block
    }

    pub fn calculate_hash(input: &String) -> String {
        let mut sha = Sha256::new();
        sha.input_str(&input);
        sha.result_str().as_bytes().to_hex()
    }

    pub fn calculate_hash_from_block(input: &Block) -> String {
        let json = serde_json::to_string(input).expect("Couldn't serialize block");
        Blockchain::calculate_hash(&json)
    }

    fn valid_proof(proof: u32, last_proof: u32) -> bool {
        let guess = format!("{}{}", proof, last_proof);
        let mut sha = Sha256::new();
        sha.input_str(&guess);
        let guess_hash = sha.result_str();
        guess_hash[0..4].eq("0000")
    }

    pub fn proof_of_work(last_proof: u32) -> u32 {
        let mut proof = 0;
        while Blockchain::valid_proof(proof, last_proof) == false {
            proof += 1;
        }
        proof
    }

    fn reward_for_proof(&mut self) {
        self.current_transactions.push( Transaction {amount: 1, recipient: self.node_identifier.clone(), sender: "0".to_string()} );
    }

    pub fn mine_new_block(&mut self) -> Block {
        let last_block = self.chain.last().unwrap().clone();
        let proof = Blockchain::proof_of_work(last_block.proof);
        let previous_hash = Blockchain::calculate_hash_from_block(&last_block);
        self.reward_for_proof();
        self.new_block(proof, previous_hash)
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
        let mut chain = Blockchain::new();

        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        assert_eq!(index, 2);
    }

    #[test]
    fn last_index_is_not_incremented() {
        let mut chain = Blockchain::new();

        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        let index2 = chain.new_transaction(Transaction { amount: 10, recipient: "you".to_string(), sender: "me".to_string() });
        assert_eq!(index, 2);
        assert_eq!(2, chain.current_transactions.len());
        assert_eq!(index2, 2);
    }

    #[test]
    pub fn test_to_hex() {
        assert_eq!("foobar".as_bytes().to_hex(), "666f6f626172");
    }

    #[test]
    pub fn test_calculate_hash()
    {
        let block = Block { index: 1, previous_hash: "1".to_string(), proof: 100, timestamp: 333037375,
            transactions: Vec::new() };
        let hash = Blockchain::calculate_hash_from_block(&block);
        let expected_hash = "36303566323332313133666639643032316337376264613461303932333666313564366136366363363062613137636335393231643564393336636139653133";
        assert_eq!(hash, expected_hash);
    }

    #[test]
    pub fn test_proof_of_work()
    {
        assert_eq!(93711, Blockchain::proof_of_work(1));
    }


    #[test]
    fn last_index_is_incremented_when_block_is_mined() {
        let mut chain = Blockchain::new();
        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        chain.mine_new_block();
        let index2 = chain.new_transaction(Transaction { amount: 10, recipient: "you".to_string(), sender: "me".to_string() });
        assert_eq!(1, chain.current_transactions.len());
        assert_eq!(index, 2);
        assert_eq!(index2, 3);
    }

}