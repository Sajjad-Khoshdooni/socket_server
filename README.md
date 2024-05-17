# Simple Implementation of Socket Server in Rust

This project implements a TCP socket server in Rust using the Tokio library. The server can handle different subscription methods (TRADES, ORDER, ALL) and respond accordingly.

## Features
* Handles incoming TCP connections asynchronously.
* Supports multiple subscription methods.
* Responds to client requests with JSON-formatted messages.

## Dependencies
* tokio: An asynchronous runtime for Rust.
* serde and serde_json: For serializing and deserializing JSON.
