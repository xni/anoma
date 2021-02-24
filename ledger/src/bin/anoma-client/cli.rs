//! The docstrings on types and their fields with `derive(Clap)` are displayed
//! in the CLI `--help`.

pub mod rpc {
    tonic::include_proto!("hello");
}
use rpc::say_client::SayClient;
use rpc::{SayRequest, SayResponse};

use anoma::cli::{ClientOpts, Gossip, InlinedClientOpts, Transfer};
use anoma::types::{Intent, Message, Transaction};
use clap::Clap;
use reqwest;
use tendermint_rpc::{Client, HttpClient};

pub async fn main() {
    match ClientOpts::parse() {
        ClientOpts::Inlined(ops) => exec_inlined(ops).await,
    }
}

async fn exec_inlined(ops: InlinedClientOpts) {
    match ops {
        InlinedClientOpts::Transfer(transaction) => transfer(transaction).await,
        InlinedClientOpts::Gossip(Gossip { orderbook, msg }) => {
            let _res = gossip(orderbook, msg).await;
        }
    }
}

async fn transfer(Transfer { src, dest, amount }: Transfer) {
    // TODO add a counter
    let tx = Transaction { src, dest, amount };
    let mut tx_bytes = vec![];
    tx.encode(&mut tx_bytes).unwrap();
    let client =
        HttpClient::new("tcp://127.0.0.1:26657".parse().unwrap()).unwrap();
    // TODO broadcast_tx_commit shouldn't be used live
    let response = client.broadcast_tx_commit(tx_bytes.into()).await;
    println!("{:#?}", response);
}

async fn gossip(
    _orderbook_addr: String,
    msg: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SayClient::connect("http://[::1]:39111").await?;
    let _response = client.send(SayRequest { name: msg }).await?;
    Ok(())
}
