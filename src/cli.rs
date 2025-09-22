use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "wtfd",
    version,
    about = "'ah shit! what was I supposed to fucking do?'...well ask me what the fuck to do."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add { description: String },
    List,
    Edit { id: u32 },
    Delete { id: u32 },
}
