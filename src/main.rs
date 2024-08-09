use chrono::Utc;
use clap::{Arg, ArgAction, Command};
use std::io::{Read, Write};
use std::process::exit;
use std::{env, fs::OpenOptions, path::PathBuf};

fn main() {
    let brag_log_dir = env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .expect("failed to get home directory")
        .join(".brag");

    std::fs::create_dir_all(brag_log_dir.clone()).expect("failed to create brag directory");

    let brag_log = brag_log_dir.join("log");

    let cmd = clap::Command::new("brag")
        .subcommand(Command::new("add").arg(Arg::new("log").action(ArgAction::Set)))
        .subcommand(Command::new("read"));

    let matches = cmd.get_matches();

    if let Some(cmd) = matches.subcommand_matches("add") {
        if let Some(input) = cmd.get_one::<String>("log") {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(brag_log)
                .expect("failed to open brag log at runtime");

            let today = Utc::now().format("%Y-%m-%d");

            writeln!(file, "{today} | {input}").expect("failed to write to brag log");
            exit(0)
        }
    }

    if let Some(_cmd) = matches.subcommand_matches("read") {
        let mut log = OpenOptions::new()
            .read(true)
            .open(brag_log)
            .expect("failed to open brag log at runtime");

        let mut content = String::new();
        log.read_to_string(&mut content)
            .expect("failed to read log to string");

        println!("{content}");
        exit(0)
    }
    exit(1)
}
