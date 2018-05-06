

fn new_block(){

}


fn hash(){

}

pub struct Transaction {
    amount: u32,
    recipient: String,
    sender: String
}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>
}

pub struct Block {
    index: u32,
    previous_hash: String,
    proof: String,
    timestamp: u32,
    transactions: Vec<Transaction>
}

impl Blockchain {
    pub fn new_transaction(&self, transaction: &Transaction){

    }
}

fn main() {
    let vec: Vec<Block> = Vec::new();
    let trans: Vec<Transaction> = Vec::new();
    let chain =  Blockchain { chain: vec, current_transactions: trans};
    let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string()};
    chain.new_transaction(&transaction);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_transaction_is_added() {
        let chain: Blockchain;
        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string()};
        chain.new_transaction(&transaction);
        assert!(0, 1);
    }
}