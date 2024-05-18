use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use socket_server_rust::{
    dto::{Request, SubscribeMethods},
    handle_all_subscription, handle_order_subscription, handle_trade_subscription
};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (socket, addrees) = listener.accept().await?;
        println!("{:#?}", addrees);
        tokio::spawn(async move {
            if let Err(e) = handle_subscription(socket).await {
                println!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_subscription(mut socket :TcpStream) -> Result<(), Box<dyn Error>>{
    let mut buffer = vec![0; 1024];

    loop{
        let n = socket.read(&mut buffer).await.unwrap();
        if n == 0 {
            return Ok(())
        } else {
            let request: Request = serde_json::from_slice(&buffer[..n])
                .unwrap_or(Request{method:SubscribeMethods::ALL});

            let response = match request.method {
                SubscribeMethods::TRADES => handle_trade_subscription(),
                SubscribeMethods::ORDER => handle_order_subscription(),
                SubscribeMethods::ALL => handle_all_subscription(),
            };
            let mut response = serde_json::to_vec(&response).unwrap();
            response.push(b'\n');

            socket.write_all(&response).await.unwrap();
        }
    }
}
