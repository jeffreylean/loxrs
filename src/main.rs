use loxrs::*;
use miette::WrapErr;
use std::cmp::Ordering;
use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
    process,
};

mod token;

fn main() -> miette::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: loxrs [script]");
            process::exit(64);
        }
        Ordering::Equal => run_file(args[1].clone().into()),
        _ => run_prompt(),
    }
}

fn run_file(path: PathBuf) -> miette::Result<()> {
    let mut content = String::new();
    let _ = BufReader::new(File::open(path).unwrap()).read_to_string(&mut content);

    run(content).wrap_err("Error executing file")?;

    Ok(())
}

fn run_prompt() -> miette::Result<()> {
    loop {
        println!("> ");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim_end().to_string();

        if line.is_empty() {
            break;
        }

        run(line).wrap_err("Error executing")?
    }
    Ok(())
}

fn run(source: String) -> miette::Result<()> {
    let lexer = token::Lexer::new(&source);
    for t in lexer {
        let token = t?;
        println!("{token}")
    }

    Ok(())
}
