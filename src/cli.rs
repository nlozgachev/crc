pub use clap::Parser;

#[derive(Parser)]
#[command(name = "crc")]
#[command(author = "Nikita Lozgachev <hello@lozgachev.dev>")]
#[command(version = "0.1.0")]
#[command(
    about = "Create React Component",
    long_about = "Create initial files for React components."
)]

pub struct Cli {
    #[arg(short, long, help = "Parent directory")]
    pub path: String,
    #[arg(value_name = "COMPONENT_NAME", help = "New component")]
    pub name: String,
}
