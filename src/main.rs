use clap::{CommandFactory, Parser};
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
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
    /// Optional output delimiter, default to human readable table output
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

    match args.delimiter {
        None => print_word_count_table(word_count),
        Some(delimiter) => print_word_count_csv(word_count, delimiter),
    }
}

fn word_count_in_buf_reader<R: BufRead>(buf_reader: R) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    for line in buf_reader.lines() {
        let word = line.unwrap();
        count.entry(word).and_modify(|x| *x += 1).or_insert(1);
    }
    count
}

fn print_word_count_table(word_count: HashMap<String, usize>) {
    let mut word_count_vec: Vec<(String, usize)> = word_count.into_iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut data = vec![];
    for (word, count) in word_count_vec.into_iter() {
        data.push(vec![word.cell(), count.cell().justify(Justify::Right)])
    }

    let separator = Separator::builder()
        .title(Some(HorizontalLine::default()))
        .column(Some(VerticalLine::default()))
        .build();
    let table = data
        .table()
        .separator(separator)
        .title(vec!["Word".cell().bold(true), "Count".cell().bold(true)])
        .bold(true);

    print_stdout(table);
}

fn print_word_count_csv(word_count: HashMap<String, usize>, delimiter: char) {
    let mut word_count_vec: Vec<(String, usize)> = word_count.into_iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}{}{}", "word", &delimiter, "count");
    for (word, count) in word_count_vec {
        println!("{}{}{}", word, &delimiter, count);
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
