//! Opt entity
//!
//! Command line options as StructOpt derive structure
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opt {
    /// Input SVD file path
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,
}
