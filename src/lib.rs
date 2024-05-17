pub mod dto;

use dto::{Response};

pub fn handle_trade_subscription() -> Response{
    Response{status: 200, content: String::from("Trade Subscribed.")}
}

pub fn handle_order_subscription() -> Response{
    Response{status: 200, content: String::from("Order Subscribed.")}
}

pub fn handle_all_subscription() -> Response{
    Response{status: 200, content: String::from("Subscribed.")}
}
