#![forbid(unsafe_code)]

use clap::Parser;
use crc::{cli::Cli, component::create_component};

fn main() {
    let cli: Cli = Cli::parse();
    let path: &str = cli.path.as_str();
    let component: &str = cli.name.as_str();
    create_component(&path, &component);
}
