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

    pub fn send_message(&self, guild_id: u32) ->  Status {
        return Status { Code: 200 };
    }
}