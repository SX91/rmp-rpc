//! This crate provides facilities to use the `MessagePack` remote procedure call system
//! (`MessagePack-RPC`) in Rust.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))]
#![cfg_attr(feature="clippy", allow(missing_docs_in_private_items))]
// #![warn(missing_docs)]

extern crate rmpv;
extern crate rmp;
extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

mod errors;
mod codec;
mod message;
mod protocol;
mod server;
mod client;

pub use server::serve;
pub use client::Client;
pub use message::Message;

/// Re-exports from [rmpv](https://docs.rs/rmpv)
pub use rmpv::{Value, Integer, Utf8String};
