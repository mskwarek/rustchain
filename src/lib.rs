fn new_block(){
}


fn hash(){
}

#[derive(PartialEq, PartialOrd)]
pub struct Transaction {
    amount: u32,
    recipient: String,
    sender: String
}


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
    pub fn new_transaction(&mut self, transaction: Transaction) -> i32{
        self.current_transactions.push(transaction);
        match self.chain.last()
            {
                Some(_) => 1,
                None => 0
            }
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
        assert_eq!(index, 0);
    }

}