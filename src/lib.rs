use colored::*;
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;

use rand::Rng;

#[derive(Deserialize, Clone)]
pub struct Question {
    pub title: String,        // Question title
    pub answers: Vec<String>, // List of answers
}

pub struct AskmeSettings {
    pub shuffle: bool,
    pub loop_questions: bool,
    pub case_sensitive: bool,
    pub show_correct: bool,
    pub wait_duration: f64,
}

#[derive(Deserialize)]
pub struct AskmeSet {
    pub title: String,            // Question title
    pub subtitle: String,         // Question subtitle
    pub questions: Vec<Question>, // List of questions
}

pub fn parse_file(filename: &str) -> Result<AskmeSet, String> {
    let yaml_file = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(e) => return Err(format!("failed to read the file to a string: {}", e)),
    };

    match serde_yaml::from_str(&yaml_file) {
        Ok(file_struct) => Ok(file_struct),
        Err(e) => Err(format!("failed to parse the file to an askme file: {}", e)),
    }
}

pub fn shuffle_arr<T: Clone>(array: &[T]) -> Vec<T> {
    let mut vec = array.to_vec();
    let arr_len = vec.len();
    let mut rng = rand::thread_rng();

    for i in 1..arr_len {
        vec.swap(arr_len - i, rng.gen_range(0..arr_len - i));
    }

    vec
}

pub fn wait_for(secs: f64) {
    std::thread::sleep(Duration::from_secs_f64(secs));
}

pub fn read_user_input() -> String {
    loop {
        let mut user_input = String::new();

        let mut stdout = io::stdout(); // try not to cry

        stdout
            .write_all("> ".as_bytes())
            .expect("you have just encountered an ultra-rare error: failed to write to the standard output!");
        stdout
            .flush()
            .expect("you have just encountered an ultra-rare error: failed to flush the standard output after writing to it!");

        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read from the standard input! did you enter an invalid character? ");

        if !user_input.trim().is_empty() {
            break user_input;
        }
    }
}

pub fn print_correct_answers(correct_count: i32, questions_total: usize) {
    println!(
        " {}",
        format!("Correct answers: {}/{}", correct_count, questions_total,).bright_purple()
    )
}
