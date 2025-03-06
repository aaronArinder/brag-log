use chrono::Utc;
use clap::{arg, Arg, ArgAction, Command};
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
    let ladder = brag_log_dir.join("ladder");

    let cmd = clap::Command::new("brag")
        .subcommand(Command::new("add")
            .args([
                arg!(-b --bucket <BUCKET> "Tag the appropriate career ladder bucket for this brag to fall under").required(true), 
                Arg::new("log").action(ArgAction::Set).required(true)
            ]))
        .subcommand(Command::new("read"))
        .subcommand(Command::new("ladder"));

    let matches = cmd.get_matches();

    if let Some(cmd) = matches.subcommand_matches("add") {
        if let Some(input) = cmd.get_one::<String>("log") {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(brag_log)
                .expect("failed to open brag log");

            let today = Utc::now().format("%Y-%m-%d");

            let ladder_bucket = cmd
                .get_one::<String>("bucket")
                .expect("failed to get required flag");

            writeln!(file, "{today} | {ladder_bucket} |{input}")
                .expect("failed to write to brag log");

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

    if let Some(_cmd) = matches.subcommand_matches("ladder") {
        let mut ladder = OpenOptions::new()
            .read(true)
            .open(ladder)
            .expect("failed to read ladder document");

        let mut content = String::new();
        ladder
            .read_to_string(&mut content)
            .expect("failed to read ladder to string");

        println!("{content}");
        exit(0)
    }

    exit(1)
}
