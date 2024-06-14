use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

/// Count the number of lines in a file or stdin
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file to read, use - to read from stdin (must not be a tty)
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let word_count;
    let mut file = args.file;

    if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        file = PathBuf::from("<stdin>");
        word_count = word_count_in_buf_reader(BufReader::new(stdin().lock()));
    } else {
        word_count = word_count_in_buf_reader(BufReader::new(File::open(&file).unwrap()));
    }

    word_count
        .iter()
        .for_each(|(word, count)| println!("{}: {}", word, count));
}

fn word_count_in_buf_reader<R: BufRead>(buf_reader: R) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    for line in buf_reader.lines() {
        let word = line.unwrap();
        count.entry(word).and_modify(|x| *x += 1).or_insert(1);
    }
    count
}
