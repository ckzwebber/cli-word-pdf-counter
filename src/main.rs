use clap::Parser;
use std::path::Path;
use pdf_extract::extract_text;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "file")]
#[command(about = "A CLI tool to count words in a pdf file", long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    if !Path::new(&cli.file).exists() {
        eprintln!("Error: File '{}' does not exist.", cli.file);
        std::process::exit(1);
    }

    println!("Processing file: {}", cli.file);

    match extract_text(&cli.file) {
        Ok(text) => {
            let cleaned_text = clean_text(&text);
            let words = cleaned_text.split_whitespace();
            let word_frequencies = count_word_frequencies(words);
            let top_words = get_top_n(&word_frequencies, 10);
            print_top_words(&top_words);
        }
        Err(e) => {
            eprintln!("Error extracting text: {}", e);
            std::process::exit(1);
        }
    }
}

fn clean_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect()
}

fn count_word_frequencies<'a>(words: impl Iterator<Item = &'a str>) -> HashMap<&'a str, u32> {
    let mut freq = HashMap::new();
    for word in words {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}

fn get_top_n(word_frequencies: &HashMap<&str, u32>, n: usize) -> Vec<(String, u32)> {
    let mut vec: Vec<(&str, u32)> = word_frequencies.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec.into_iter().take(n).map(|(k, v)| (k.to_string(), v)).collect()
}

fn print_top_words(top_words: &[(String, u32)]) {
    println!("Top {} most frequent words:", top_words.len());
    for (word, count) in top_words {
        println!("{} - {}x", word, count);
    }
}