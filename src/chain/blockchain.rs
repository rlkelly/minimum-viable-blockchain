use super::address::Public;
use super::block::Block;
use super::transaction::SignedTransaction;

#[derive(Debug, Clone)]
pub struct BlockChain {
    pub current_block: Block,
    pub prev_blocks: Vec<Block>,
    // TODO: nodes
}

impl BlockChain {
    pub fn new(address: Public) -> BlockChain {
        let genesis: Block = Block::genesis();
        let current_block = Block::new(&genesis, address);
        let prev_blocks: Vec<Block> = [genesis].to_vec();

        BlockChain {
            prev_blocks,
            current_block,
        }
    }

    pub fn add_current_block(&mut self, address: Public) {
        let new_block = Block::new(&self.current_block, address);
        let old_block = self.current_block.clone();
        self.prev_blocks.push(old_block);
        self.current_block = new_block;
    }

    pub fn receive_new_block(&mut self, block: Block, address: Public) -> bool {
        if block.verify(self) {
            self.current_block = Block::new(&block, address);
            self.prev_blocks.push(block);
            return true;
        }
        false
    }

    pub fn add_transaction(&mut self, transaction: SignedTransaction) -> bool {
        if transaction.verify(self) {
            if !self.current_block.transactions.contains(&transaction) {
                self.current_block.transactions.push(transaction);
                self.current_block.update_header();
                return true;
            }
        }
        false
    }

    pub fn verify(&self) -> bool {
        for block in &self.prev_blocks {
            if block.verify(self) == false {
                return false;
            }
        }
        true
    }

    pub fn prove_work(&mut self) -> bool {
        self.current_block.prove_work()
    }
}
