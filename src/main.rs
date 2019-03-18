#[macro_use]
extern crate log;
extern crate env_logger;

extern crate futures;

extern crate tokio;
extern crate tokio_io;
extern crate tokio_fs;
extern crate tokio_codec;
extern crate tokio_process;

use futures::Future;
use tokio::prelude::*;

fn conversion(line: String) -> String {
    let mut s = String::from("Converted: ");
    s.push_str(&line);
    s.push_str("\n");
    s
}

fn main() {
    env_logger::Builder::new()
        .target(env_logger::Target::Stderr)
        .filter(None, log::LevelFilter::Debug)
        .init();

    debug!("Debug messages enabled");
    info!("Info messages enabled");
    warn!("Warning messages enabled");
    error!("Error messages enabled");

    let input = tokio_fs::stdin();

    let server = tokio::io::lines(std::io::BufReader::new(input))
        .map_err(|e| error!("{:?}", e))
        .map(conversion)
        .for_each(move |line| {
            let output = tokio_fs::stdout();
            tokio::io::write_all(output, line)
                .map_err(|e| error!("{:?}", e))
                .map(|_| {})
        });

    info!("Creating Runtime");
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    info!("Finished creating Runtime");

    runtime.block_on(server).unwrap();

    runtime.shutdown_now().wait().unwrap();
    debug!("Finished running future");
}

