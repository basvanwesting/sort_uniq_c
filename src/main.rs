use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

/// Efficient version of "sort | uniq -c" with some output options.
/// Output order is word, count. Sorted by descending count.
#[derive(Parser)]
struct Cli {
    /// Optional output delimiter, default to human readable aligned text output
    #[arg(short, long)]
    delimiter: Option<char>,
    /// The path to the file to read, use - to read from stdin (must not be a tty)
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;

    let word_count = if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        word_count_in_buf_reader(BufReader::new(stdin().lock()))
    } else {
        word_count_in_buf_reader(BufReader::new(File::open(&file).unwrap()))
    };

    print_word_count(word_count, args.delimiter);
}

fn word_count_in_buf_reader<R: BufRead>(buf_reader: R) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    buf_reader.lines().for_each(|line| {
        let word = line.unwrap();
        count.entry(word).and_modify(|x| *x += 1).or_insert(1);
    });
    count
}

fn print_word_count(word_count: HashMap<String, usize>, delimiter: Option<char>) {
    let mut word_count_vec: Vec<(&String, &usize)> = word_count.iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    match delimiter {
        Some(delimiter) => {
            word_count_vec.iter().for_each(|(word, count)| {
                println!("{}{}{}", word, delimiter, count);
            });
        }
        None => {
            let max_word_length = word_count_vec
                .iter()
                .map(|(word, _)| word.len())
                .max()
                .unwrap_or(0);

            let max_count_length = word_count_vec
                .iter()
                .map(|(_, count)| count.to_string().len())
                .max()
                .unwrap_or(0);

            word_count_vec.iter().for_each(|(word, count)| {
                println!(
                    "{:<word_width$} {:>count_width$}",
                    word,
                    count,
                    word_width = max_word_length,
                    count_width = max_count_length
                )
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Cursor;

    #[test]
    fn test_word_count_in_buf_reader() {
        let data = "word1\nword2\nword1\nword3\nword2\nword2";
        let cursor = Cursor::new(data);
        let result = word_count_in_buf_reader(cursor);

        let mut expected = HashMap::new();
        expected.insert("word1".to_string(), 2);
        expected.insert("word2".to_string(), 3);
        expected.insert("word3".to_string(), 1);

        assert_eq!(result, expected);
    }
}
