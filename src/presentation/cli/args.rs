use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
#[command(name = "switchbot-cli")]
pub struct CliArgs {
    #[arg(long)]
    pub device: String,

    #[arg(long)]
    pub command: String,

    #[arg(long)]
    pub value: Option<String>
}

#[derive(Parser, Debug)]
#[command(name="switchbot-cli")]
#[command(about = "CLI for controlling SwitchBot devices", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    DeviceList,
    Exec {
        #[arg(long)]
        device: String,

        #[arg(long)]
        command: String,

        #[arg(long)]
        value: Option<String>
    }
}