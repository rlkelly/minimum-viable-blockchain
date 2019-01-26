use super::block::Block;
use super::address::Public;


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

    pub fn verify(&self) -> bool {
        for block in &self.prev_blocks {
            if block.verify() == false {
                return false
            }
        }
        true
    }
}
