#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate futures;

extern crate tokio;
extern crate tokio_io;
extern crate tokio_fs;
extern crate tokio_codec;
extern crate tokio_process;

use futures::Future;
use tokio::prelude::*;

fn main() {
    env_logger::Builder::new()
        .target(env_logger::Target::Stderr)
        .filter(None, log::LevelFilter::Debug)
        .init();

    debug!("Debug messages enabled");
    info!("Info messages enabled");
    warn!("Warning messages enabled");
    error!("Error messages enabled");

    let codec = tokio::codec::LinesCodec::new();

    let input = tokio_fs::stdin();
    let output = tokio_fs::stdout();

    let copied = tokio::io::copy(input, output).map(|amt| {
        debug!("Wrote {:?} bytes", amt);
    }).map_err(|err| {
        error!("Error: {:?}", err);
    });

    info!("Creating Runtime");
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    info!("Finished creating Runtime");

    runtime.block_on(copied).unwrap();

    runtime.shutdown_now().wait().unwrap();
    debug!("Finished running future");
}

