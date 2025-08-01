use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "switchbot-cli")]
#[command(about = "CLI for controlling SwitchBot devices", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    Exec {
        #[arg(short, long)]
        device: String,

        #[arg(short='c', long)]
        command: String,

        #[arg(short, long, num_args=1..)]
        values: Option<Vec<String>>,

        #[arg(short='C', long)]
        customize: bool,
    },
}
