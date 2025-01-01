#![allow(clippy::multiple_crate_versions)]

use clap::{Arg, ArgMatches, Command};
use jah::Jah;
mod jah;

fn app() -> ArgMatches {
    Command::new("jah")
        .arg(
            Arg::new("commit")
                .short('c')
                .long("commit")
                .num_args(0)
                .help("Run test and commit on success"),
        )
        .arg(
            Arg::new("push")
                .short('p')
                .long("push")
                .num_args(0)
                .help("Send modifications to remotes"),
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .num_args(0)
                .help("Show last 5 last log"),
        )
        .get_matches()
}
fn main() {
    let app = app();
    if app.get_flag("commit") {
        Jah::commit();
    } else if app.get_flag("push") {
        Jah::send();
    } else if app.get_flag("log") {
        Jah::log();
    }
}
