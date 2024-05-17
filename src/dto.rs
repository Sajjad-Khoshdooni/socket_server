use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeMethods{
    TRADES,
    ORDER,
    ALL
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request{
    pub method: SubscribeMethods
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub status: usize,
    pub content: String
}

