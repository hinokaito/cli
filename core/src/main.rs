use clap::{Parser, Subcommand};
use std::{
    fs::{self, File},
    path,
    io::{BufRead, BufReader},
};

#[derive(Parser)]
#[command(version, about = "Made of Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,    
}

#[derive(Subcommand)]
enum Commands {
    Echo {
        #[arg(name = "TEXT")]
        text: Vec<String>,

        #[arg(short = 'n', long)]
        omit_newline: bool,
    },
    Cat {
        #[arg(name = "PATH")]
        paths: Vec<path::PathBuf>,

        #[arg(short = 'n', long)]
        display_lines: bool,
    },
    Grep {
        #[arg(name = "KEY_WORD")]
        key_word: String,

        #[arg(name = "PATH")]
        path: path::PathBuf,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Echo { text, omit_newline }) => {
            print!("{}", text.join(" "));
            if !omit_newline {
                println!();
            }
        }
        Some(Commands::Cat { paths, display_lines }) => {
            for path in paths.iter() {
                println!("--{:?}----", path);
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    if *display_lines {
                        for (line, sentence) in reader.lines().enumerate() {
                            match sentence {
                                Ok(sentence) => println!("{}| {}", line+1, sentence),
                                Err(_) => break,
                            }
                        } 
                    } else {
                        for sentence in reader.lines() {
                            match sentence {
                                Ok(sentence) => println!("{}", sentence),
                                Err(_) => break,
                            }
                        } 
                    }
                } else {
                    return
                }
                println!();
            }
        }
        Some(Commands::Grep { key_word, path }) => {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for (line, sentence) in reader.lines().enumerate() {
                    match sentence {
                        Ok(sentence) if sentence.contains(key_word) => println!("HIT! line{}| {}", line, sentence),
                        _ => {}
                    }
                }
            }
        }
        _ => {
            println!("未実装のコマンドです");
        }
    }
}