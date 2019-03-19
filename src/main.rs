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
    let mut s = String::from("Sent to REPL: ");
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


    let Process {
        input,
        output,
        child
    } = create_repl(&["nix", "repl", "<nixpkgs/nixos>"]);

    let forwarder = tokio::io::lines(std::io::BufReader::new(tokio_fs::stdin()))
        .map_err(|e| error!("{:?}", e))
        .map(conversion)
        .for_each(|line| {
            tokio::io::write_all(output, line)
                .map_err(|e| error!("{:?}", e))
                .map(|_| {})
        });

    info!("Creating Runtime");
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    info!("Finished creating Runtime");

    runtime.spawn(forwarder);
    runtime.block_on(child).unwrap();

    runtime.shutdown_now().wait().unwrap();
    debug!("Finished running future");
}

use std::ffi::OsStr;

fn create_repl(command: &[&str]) -> Process {
    use std::process::{Command, Stdio};
    use tokio_process::CommandExt;

    let mut child = {
        let mut child = Command::new(OsStr::new(command[0]));

        for arg in command.iter().skip(1) {
            child.arg(arg);
        }

        child.stdin(Stdio::piped())
            .stdout(Stdio::piped());

        child.spawn_async().unwrap()
    };

    let input = child.stdin().take().unwrap();

    let output = tokio::io::lines(std::io::BufReader::new(child.stdout().take().unwrap()))
        .map_err(|e| error!("{:?}", e))
        .map(conversion);

    Process {
        input: input,
        output: Box::new(output),
        child: child
    }
}

struct Process {
    input: tokio_process::ChildStdin,
    output: Box<dyn stream::Stream<Item=String, Error=()>>,
    child: tokio_process::Child
}
