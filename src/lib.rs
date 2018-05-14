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
    pub fn new_transaction(&mut self, transaction: Transaction) -> u32{
        self.current_transactions.push(transaction);
        let num = match self.chain.last()
            {
                Some(_) => self.chain.last().unwrap().index,
                None => 0
            };
        num+1
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

}