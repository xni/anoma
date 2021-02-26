//! The docstrings on types and their fields with `derive(Clap)` are displayed
//! in the CLI `--help`.

use anoma::{
    cli::{InlinedNodeOpts, NodeOpts},
    config::Config,
};
use clap::Clap;

use crate::gossip;
use crate::shell;

pub fn main(config: Config) {
    let NodeOpts { base_dir, rpc, ops } = NodeOpts::parse();
    let config = base_dir.map(|dir| Config::new(dir)).unwrap_or(config);
    exec_inlined(config, rpc, ops)
}

fn exec_inlined(config: Config, rpc: bool, ops: InlinedNodeOpts) {
    let _exec = match ops {
        InlinedNodeOpts::RunOrderbook(arg) => {
            gossip::run(config, rpc, arg.local_address, arg.peers, arg.topics)
        }
        InlinedNodeOpts::RunAnoma => Ok(shell::run(config)),
        InlinedNodeOpts::ResetAnoma => Ok(shell::reset(config)),
    };
}
