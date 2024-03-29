mod cmds;
mod error;

use std::collections::HashSet;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use regex::Regex;
use rand::seq::IteratorRandom;
use lazy_static::lazy_static;

lazy_static! {
    static ref CHANGE_DIR: Regex = Regex::new("^[a-zA-Z]:.*$").unwrap();
}

fn main() {

    print!("{}[2J", 27 as char);

    let mut rng = rand::thread_rng();

    let mut known_commands = HashSet::new();
    for cmd in cmds::COMMANDS {
        known_commands.insert(cmd.to_string());
    }

    let mut errors = HashSet::new();
    for err in error::ERRORS {
        errors.insert(err.to_string());
    }

    println!("Starting MS-DOS...");
    println!();

    let mut rl = Editor::<()>::new();
     loop {
        let readline = rl.readline("B:\\>");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let line = line.as_str().trim();
                let lines: Vec<String> = line.split_ascii_whitespace().map(|x| x.to_owned()).collect();
                let command = lines.first().unwrap().to_owned();
                rl.add_history_entry(line);
                if line.trim() == "hal9000" {
                    break;
                } else if CHANGE_DIR.is_match(&command) {
                    println!("Not ready reading drive {}", command.chars().next().unwrap());
                    abort_retry_fail(&mut rl);
                } else if command == "help" {
                    for cmd in cmds::COMMANDS {
                        println!("{}", cmd);
                        std::thread::sleep(std::time::Duration::from_millis(2));
                    }
                } else if known_commands.contains(&command) {
                    println!();
                    println!("{}", errors.clone().into_iter().choose(&mut rng).unwrap());
                } else {
                    println!("Unknown command {}", line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
            },
            Err(ReadlineError::Eof) => {
                print!("^D ");
            },
            Err(_) => {
                println!("Error");
            }
        }
    }
}

fn abort_retry_fail(rl: &mut Editor<()>) {
    loop {
        let readline = rl.readline("Abort, Retry, Fail?");
        if let Ok(line) = readline {
            if line.as_str() == "Abort" {
                break
            }
        }
    }
}