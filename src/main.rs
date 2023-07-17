mod ruscord;

use crate::ruscord::client::{ Client };

fn main(){
    
    let c: Client = Client {
        token: String::from("paksdpoaksdpoaksdpokas"),
        max_shards: 10
    };
}