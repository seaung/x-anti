use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version="1.0.0", author="seaung", about="x-anti command line tools")]
pub struct Cli {
    #[arg(short, long, value_name="FILE")]
    config: Option<std::path::PathBuf>,

    #[arg(short, long)]
    mode: bool,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Check {
        #[arg(short, long)]
        targer: String,
        #[arg(short, long)]
        path: String,
    },
    Exp {
        #[arg(short, long)]
        target: String,
    }
}

pub fn start() {}
