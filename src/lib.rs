use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId, Timestamp, near_bindgen};
use near_sdk::collections::{Vector};
use witgen::witgen;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
pub struct PostInfo {   // structure of each post info
    pub name: String,
    pub description: String,
    pub authorId: AccountId,
    pub timestamp: Timestamp,
} 

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Contract {
    post: Vector<PostInfo>,     // create vector of postInfo named post
}

impl Default for Contract {
    fn default() -> Self {
        Self{post: Vector::new(b"m")}
    }
}

#[near_bindgen]
impl Contract {
    
    // Add post function
    pub fn add_post(&mut self, name:String, description: String) {
        let authorId = env::predecessor_account_id();                       // Get the post author's userId
        let timestamp = env::block_timestamp();                             // Get block's timestamp
        
        let postInfo = PostInfo{name, description, authorId, timestamp};
        self.post.push(&postInfo);                                          // Push all the infomation to the vector
    }

    // Get Post function
    pub fn get_post(&self, limit: Option<u64>) -> Vec<PostInfo> {
        self.post.iter()                                                    // Iterate over post 
        .take(limit.unwrap_or(10) as usize)                                 // take n limit post 
        .collect()                                                          // collect return post
    }

    pub fn total_post(&self) -> u64 { self.post.len() }                     // get total number of post
}
