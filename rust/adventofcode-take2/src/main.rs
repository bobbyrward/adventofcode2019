#[macro_use]
mod args;
mod command;
mod intcode;
mod point;

use anyhow::{Context, Result};
use clap::Clap;
use tracing_subscriber::FmtSubscriber;

use crate::command::Command;

// NOTE: Each solution module must be added here
solution!(day01, day02, day03, day04, day05, day06, day07, day09, day11);

fn input(name: &str) -> Result<String> {
    Ok(
        std::fs::read_to_string(format!("inputs/{}.txt", name))
            .with_context(|| name.to_string())?,
    )
}

fn main() -> Result<()> {
    let args = args::Args::parse();

    FmtSubscriber::builder()
        .with_env_filter(args.env_filter())
        .init();

    let solution = args.command.execute()?;

    println!("Solution:\n{}", solution);

    Ok(())
}
