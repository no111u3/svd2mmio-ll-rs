use svd2mmio_ll_rs::opt::Opt;

use structopt::StructOpt;

use std::fs::File;
use std::io::Read;
use std::process;

use log::{error, info};

use svd_parser as svd;

use anyhow::{Context, Result};

fn run() -> Result<()> {
    let opt = Opt::from_args();

    setup_logging();

    let input = opt.input;

    info!("input: {:?}", input);

    let svd_xml = &mut String::new();
    File::open(input)
        .context("Cannot open the SVD file")?
        .read_to_string(svd_xml)
        .context("Cannot read the SVD file")?;

    let device = svd::parse(svd_xml)?;

    info!("device: {}", device.name);

    Ok(())
}

fn setup_logging() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp(None);

    let log_lvl_from_env = std::env::var_os(env_logger::DEFAULT_FILTER_ENV).is_some();

    if log_lvl_from_env {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        let level = log::LevelFilter::Info;
        log::set_max_level(level);
        builder.filter_level(level);
    }

    builder.init();
}

fn main() {
    if let Err(ref e) = run() {
        error!("{:?}", e);

        process::exit(1);
    }
}
