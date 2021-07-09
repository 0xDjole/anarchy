use crypto::{digest::Digest, sha2::Sha256};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new () -> Self {
        let genesis_block = Block{
            index:0,
            previous_hash: String::from(""),
            hash: String::from('0')
        };

        Blockchain {
            blocks: vec![genesis_block] as Vec<Block>
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    index: i32, 
    previous_hash: String,
    hash: String,
}


impl Blockchain {
    pub fn create_block(&mut self) -> Option<Block> {      
        let previous_block = self.get_previous_block();

        if previous_block.is_none() {
            return None
        }

        
        let previous_block_data = previous_block.unwrap();      

        let mut hasher = Sha256::new();
        let utc: DateTime<Utc> = Utc::now();      
        
        hasher.input_str(&previous_block_data.hash);
        hasher.input_str(&previous_block_data.index.to_string());
        hasher.input_str(&utc.to_string());
        let hash = hasher.result_str();

       let new_block =  Block {
            index: self.get_blocks_length() as i32 + 1,
            previous_hash: previous_block_data.hash.clone(),
            hash
        };

        self.blocks.push(new_block.clone());

        Some(new_block)
    }

    pub fn get_previous_block(&self)  -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_blocks_length(&self)  -> usize {
         self.blocks.len()
    }
}
