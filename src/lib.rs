// Copyright 2023 Eason Qin <eason@ezntek.com> and Cikitta Tjok <daringcuteseal@gmail.com>.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use colored::*;
use inquire::{Confirm, InquireError, Text};
use rand::seq::SliceRandom;
use std::time::Duration;

pub mod errors;
pub mod prelude;
pub mod traits;
pub mod types;

pub fn shuffle_arr<T: Clone>(array: &[T]) -> Vec<T> {
    let mut vec = array.to_vec();
    vec.shuffle(&mut rand::thread_rng());
    vec
}

pub fn wait_for(secs: f64) {
    std::thread::sleep(Duration::from_secs_f64(secs));
}

pub fn get_input() -> String {
    loop {
        let user_input = match Text::new("Answer:").prompt() {
            Ok(text) => text,
            Err(err) => {
                if let InquireError::OperationInterrupted = err {
                    std::process::exit(1)
                } else {
                    panic!("Failed to read from stdin!")
                }
            }
        };
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

pub fn print_warning(msg: &str) {
    println!("{} {}", "[!] warning:".bold(), msg.yellow());
}

pub fn print_err(msg: &str) {
    println!("{} {}", "[!!] error:".red(), msg);
}

pub fn print_info(msg: &str) {
    println!("[i] info: {}", msg.bold())
}

pub fn get_yn_from_input() -> bool {
    match Confirm::new("Answer:").prompt() {
        Ok(true) => true,
        Ok(false) => false,
        Err(err) => {
            if let inquire::InquireError::OperationInterrupted = err {
                std::process::exit(1)
            } else {
                panic!("Failed getting user input!")
            }
        }
    }
}
