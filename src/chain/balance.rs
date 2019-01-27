use super::address::Public;
use super::blockchain::BlockChain;
use super::header::COINBASE_VALUE;

pub struct Account {
    pub address: Public,
    pub balance: f64,
}

#[allow(dead_code)]
impl Account {
    #[allow(dead_code)]
    pub fn new(address: Public, blockchain: &BlockChain) -> Account {
        let mut account = Account {
            address,
            balance: 0.0,
        };
        account.update_balance(blockchain);
        account
    }

    #[allow(dead_code)]
    pub fn update_balance(&mut self, blockchain: &BlockChain) {
        let sent_transactions: f64 = blockchain.prev_blocks.iter()
            .map(|block| block.transactions.iter()
                .filter(|st| st.transaction.sender == self.address)
                .fold(0.0, |sum, st| sum + st.transaction.amount)
            )
            .fold(0.0, |sum, total| sum + total);
        let received_transactions: f64 = blockchain.prev_blocks.iter()
            .map(|block| block.transactions.iter()
                .filter(|st| st.transaction.receiver == self.address)
                .fold(0.0, |sum, st| sum + st.transaction.amount)
            )
            .fold(0.0, |sum, total| sum + total);
        let coinbase = blockchain.prev_blocks.iter()
            .filter(|block| block.header.coinbase == self.address)
            .count() as f64;
        self.balance = received_transactions + coinbase * COINBASE_VALUE - sent_transactions;
    }
}
