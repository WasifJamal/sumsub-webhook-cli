// src/cli.rs
use crate::payload::predefined::EventKind;
use clap::{Parser, Subcommand};

/// CLI arguments for the Sumsub webhook simulation tool
#[derive(Parser)]
#[command(author, version, about = "Sumsub Webhook CLI", long_about = None)]
pub struct Cli {
    /// Specifies the command to run (predefined or custom payload)
    #[command(subcommand)]
    pub cmd: Command,

    /// Your Sumsub webhook secret used for signing the payload
    #[arg(short, long)]
    pub secret: String,

    /// The URL of your local webhook endpoint to send the request to
    #[arg(short, long)]
    pub url: String,
}

#[derive(Subcommand)]
pub enum Command {
    /// Send a predefined Sumsub event
    Event {
        /// Which predefined event to simulate (e.g. applicant-created, applicant-reviewed)
        #[arg(short, long, value_enum)]
        event: EventKind,
    },

    /// Send a custom payload loaded from a JSON file
    Payload {
        /// Path to your payload.json file
        #[arg(short, long)]
        path: String,
    },
}
