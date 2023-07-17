use std::{thread, time};

use crate::ruscord::status::Status;

pub struct Client {
    pub token: String,
    pub max_shards: u8
}

impl Client {
    pub fn get_token(&self) -> String {
        return self.token.clone();
    }

    pub fn get_shards(&self) -> u8 {
        return self.max_shards;
    }

    // Message Actions

    pub fn send_message(&self, channel_id: u32, message_content: String) {
        let c = reqwest::Client::new();
        let res = c.post("https://")
    }

    pub fn login(&self, delay: u64) {
        let dur = time::Duration::from_secs(delay);
        thread::sleep(dur);
        println!("Slept for {} seconds!", delay);
    }
}