use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Client{
    pub address: SocketAddr,
    pub subscribed_method: SubscribeMethods
}

impl Client{
    pub fn new(socket_addr: SocketAddr, subscribed_method: SubscribeMethods) -> Client{
        Client{ address:socket_addr, subscribed_method }
    }
}

#[derive(Debug)]
pub struct Server{
    pub clients: Vec<Client>
}

impl Server{
    pub fn new() -> Server{

        Server{
            clients: Vec::new()
        }
    }

    pub fn add_client(&mut self, socket_addr: SocketAddr, subscribed_method: SubscribeMethods){
        let client = Client::new(socket_addr, subscribed_method);
        self.push_client(client);
    }

    pub fn push_client(&mut self, client: Client){
        if !self.clients.contains(&client) {
            self.clients.push(client);
        }
    }
}
