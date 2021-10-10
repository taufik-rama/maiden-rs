mod config;
mod fixture;
mod service;

use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

/// Main command definition
#[derive(StructOpt, Debug)]
#[structopt(name = "maiden", about = "a helper tools to automate things")]
struct Command {
    /// Config file location
    #[structopt(short = "c", long, parse(from_os_str))]
    config_file: Option<PathBuf>,

    /// Output file for service generation
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(subcommand)]
    subcmd: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    /// Handles fixture data processing
    Fixture(Fixture),

    /// Handles service generations
    Service(Service),
}

#[derive(StructOpt, Debug)]
enum Fixture {
    /// Push the fixtures into storage
    Push,
}

#[derive(StructOpt, Debug)]
struct Service {
    /// Service status
    #[structopt(short, long)]
    status: bool,

    #[structopt(subcommand)]
    subcmd: Option<ServiceSubcommand>,
}

#[derive(StructOpt, Debug)]
enum ServiceSubcommand {
    /// Generate service implementation
    Generate,

    /// Run generated service
    Run,
}

#[derive(Error, Debug)]
enum MaidenError {
    /// CLI parse error
    #[error("{0}")]
    Parse(#[from] structopt::clap::Error),

    /// Configuration file error
    #[error("configuration error: {0}")]
    Config(config::ConfigError),

    /// Fixture execution error
    #[error("fixture error: {0}")]
    Fixture(fixture::FixtureError),

    /// Service generation error
    #[error("service error: {0}")]
    Service(service::ServiceError),
}

async fn run() -> Result<(), MaidenError> {
    let cmd = Command::from_args_safe()?;

    let config = config::Config::new(cmd.config_file).await?;
    println!("{:#?}", config);

    match cmd.subcmd {
        Subcommand::Fixture(_) => {}
        Subcommand::Service(_) => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
