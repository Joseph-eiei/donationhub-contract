use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, Timestamp};
use witgen::witgen;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
pub struct PostInfo {
    // structure of each post info
    pub name: String,
    pub description: String,
    pub authorId: AccountId,
    pub timestamp: Timestamp,
    pub eth_address: String,
    pub bitkub_address: String,
    pub jfin_address: String,
    pub total_raised: u64,
    pub id: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Contract {
    post: Vector<PostInfo>, // create vector of postInfo named post
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            post: Vector::new(b"m"),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn add_post(
        &mut self,
        name: String,
        description: String,
        eth_address: Option<String>,
        bitkub_address: Option<String>,
        jfin_address: Option<String>,
        total_raised: Option<u64>,
    ) {
        let authorId = env::predecessor_account_id(); // Get the post author's userId
        let timestamp = env::block_timestamp(); // Get block's timestamp
        let id = self.post.len();

        let postInfo = PostInfo {
            name,
            description,
            authorId,
            timestamp,
            eth_address: eth_address.unwrap_or_default(),
            bitkub_address: bitkub_address.unwrap_or_default(),
            jfin_address: jfin_address.unwrap_or_default(),
            total_raised: total_raised.unwrap_or_default(),
            id,
        };
        self.post.push(&postInfo); // Push all the infomation to the vector
    }

    pub fn update_total_raised(&mut self, fund: u64, postId: u64) {
        let mut new_post: PostInfo = self
            .post
            .get(postId)
            .unwrap_or_else(|| panic!("Post id {} not found", postId));

        new_post.total_raised += fund;

        self.post.replace(postId, &new_post);
    }

    pub fn get_post(&self, limit: Option<u64>) -> Vec<PostInfo> {
        self.post
            .iter() // Iterate over post
            .take(limit.unwrap_or(10) as usize) // take n limit post
            .collect() // collect return post
    }

    pub fn get_post_by_postId(&self, postId: u64) -> PostInfo {
        self.post
            .get(postId)
            .unwrap_or_else(|| panic!("Post {} not found", postId))
    }

    pub fn total_post(&self) -> u64 {
        self.post.len()
    }
}
