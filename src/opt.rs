//! Opt entity
//!
//! Command line options as StructOpt derive structure
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opt {}
