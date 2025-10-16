use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/// Hash Chain Log Entry for immutable audit trails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChainEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub previous_hash: String,
    pub current_hash: String,
    pub nonce: u64,
}

pub struct HashChainLogger {
    chain: Vec<HashChainEntry>,
}

impl HashChainLogger {
    pub fn new() -> Self {
        let mut logger = Self {
            chain: Vec::new(),
        };
        
        // Create genesis block
        logger.add_genesis_block();
        logger
    }
    
    fn add_genesis_block(&mut self) {
        let genesis = HashChainEntry {
            id: "genesis".to_string(),
            timestamp: Utc::now(),
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            current_hash: self.calculate_hash("genesis", "Genesis Block", "0", 0),
            nonce: 0,
        };
        
        self.chain.push(genesis);
    }
    
    pub fn add_entry(&mut self, data: String) -> Result<String, crate::core::HimsError> {
        let previous_entry = self.chain.last().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let nonce = 0; // In a real implementation, this would be calculated through proof-of-work
        
        let current_hash = self.calculate_hash(&id, &data, &previous_entry.current_hash, nonce);
        
        let entry = HashChainEntry {
            id: id.clone(),
            timestamp,
            data,
            previous_hash: previous_entry.current_hash.clone(),
            current_hash,
            nonce,
        };
        
        self.chain.push(entry);
        Ok(id)
    }
    
    fn calculate_hash(&self, id: &str, data: &str, previous_hash: &str, nonce: u64) -> String {
        let input = format!("{}{}{}{}", id, data, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn verify_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            // Verify current hash
            let calculated_hash = self.calculate_hash(
                &current.id,
                &current.data,
                &current.previous_hash,
                current.nonce,
            );
            
            if calculated_hash != current.current_hash {
                return false;
            }
            
            // Verify link to previous block
            if current.previous_hash != previous.current_hash {
                return false;
            }
        }
        
        true
    }
    
    pub fn get_chain(&self) -> &Vec<HashChainEntry> {
        &self.chain
    }
}

impl Default for HashChainLogger {
    fn default() -> Self {
        Self::new()
    }
}