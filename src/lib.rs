use colored::*;
use figlet_rs::FIGfont;
use serde::Deserialize;
use std::{
    io::{self, Write},
    process::exit,
};

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

#[derive(Deserialize)]
pub struct Question {
    pub title: String,
    pub answers: Vec<String>,
}

#[derive(Deserialize)]
pub struct AskMeContent {
    pub title: String,
    pub subtitle: String,
    pub shuffle: bool,
    pub loop_questions: bool,
    pub case_sensitive: bool,
    pub show_correct: bool,
    pub questions: Vec<Question>,
}

pub fn raise_user_err(message: &str) {
    println!("{}", format!("{}{}", "ERROR: ", message).red());
    exit(1);
}

pub fn read_user_input() -> String {
    loop {
        print!("> ");
        let mut user_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        if !user_input.is_empty() {
            break user_input;
        }
    }
}
pub struct App<'a> {
    pub askme_content: &'a AskMeContent,
    pub q_index: usize,
    pub correct_count: i32,
}

impl App<'_> {
    pub fn print_title(&self) {
        let font = FIGfont::standard().unwrap();
        let title_text = font
            .convert(&self.askme_content.title)
            .unwrap()
            .to_string()
            .cyan();

        println!("{}", title_text);
    }

    pub fn print_subtitle(&self) {
        println!(" {}", self.askme_content.subtitle.blue());
    }

    pub fn print_question(&self) {
        println!("{}", self.askme_content.questions[self.q_index].title);
    }

    pub fn check_for_empty_questions(&self) -> bool {
        self.askme_content.questions.is_empty()
    }

    pub fn print_answers(&self) {
        let answers = &self.askme_content.questions[self.q_index].answers;
        if answers.len() > 1 {
            println!(" {}", "The correct answers are:".bold());
            for answer in answers {
                println!(" {}", format!("• {}", answer).bold());
            }
            println!();
        } else {
            println!(
                " {}",
                format!("The correct answer is: {}\n", answers[0]).bold()
            );
        }
    }

    pub fn print_correct(&self) {
        println!(
            " {}",
            format!(
                "Correct answers: {}/{}",
                self.correct_count,
                self.askme_content.questions.len()
            )
            .bright_purple()
        )
    }
    pub fn ask_question_routine(&mut self) {
        /* Print question */
        self.print_question();

        /* Get user input */
        let mut user_answer = read_user_input();

        /* Turn to lowercase if case sensitive is set to false */
        if !self.askme_content.case_sensitive {
            user_answer = user_answer.to_lowercase();
        }

        let mut tmp_is_correct = false;

        /* Check answer */
        for answer in &self.askme_content.questions[self.q_index].answers {
            if *user_answer.trim() == *answer.to_lowercase() {
                println!("{}\n", CORRECT_FEEDBACK_STR.green());
                self.correct_count += 1;
                tmp_is_correct = true;
            }
        }
        if !tmp_is_correct {
            println!("{}\n", INCORRECT_FEEDBACK_STR.red());
            if self.askme_content.show_correct {
                self.print_answers();
            }
        }

        if self.q_index + 1 == self.askme_content.questions.len()
            && self.askme_content.loop_questions
        {
            self.q_index = 0; // Go back to index 0 if we are looping
        }

        self.q_index += 1;
    }

    pub fn main_loop(&mut self) {
        self.q_index = 0;

        if self.check_for_empty_questions() {
            raise_user_err("Empty questions!");
        };

        /* Print title */
        self.print_title();

        /* Print subtitle */
        self.print_subtitle();

        /* Start the loop */
        if self.askme_content.loop_questions {
            loop {
                self.ask_question_routine();
            }
        } else {
            while self.q_index < self.askme_content.questions.len() {
                self.ask_question_routine();
            }
        }

        self.print_correct();
    }
}
