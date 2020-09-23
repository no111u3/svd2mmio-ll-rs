use svd2mmio_ll_rs::{generate, opt::Opt};

use structopt::StructOpt;

use std::fs::File;
use std::io::{Read, Write};
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

    let mut device_x = String::new();
    let items = generate::device::render(&device, &mut device_x)?;
    let mut file = File::create("lib.rs").expect("Couldn't create lib.rs file");

    let data = items.to_string().replace("] ", "]\n");
    file.write_all(&data.as_ref())
        .expect("Couldn't write code to lib.rs");

    writeln!(File::create("device.x")?, "{}", device_x)?;

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
