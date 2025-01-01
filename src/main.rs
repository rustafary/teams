#![allow(clippy::multiple_crate_versions)]

use clap::{ArgMatches, Command};
use jah::Jah;
mod jah;

fn app() -> ArgMatches {
    Command::new("jah")
        .subcommand(Command::new("commit"))
        .subcommand(Command::new("push"))
        .subcommand(Command::new("log"))
        .get_matches()
}
fn main() {
    let app = app();
    if app.subcommand_matches("commit").is_some() {
        Jah::commit();
    } else if app.subcommand_matches("push").is_some() {
        Jah::send();
    } else if app.subcommand_matches("log").is_some() {
        Jah::log();
    }
}
