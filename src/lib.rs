use colored::*;
use figlet_rs::FIGfont;
use rand::{rngs::ThreadRng, Rng};
use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use std::{
    io::{self, Write},
    process::exit,
};

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

#[derive(Deserialize, Clone)]
pub struct Question {
    pub title: String,        // Question title
    pub answers: Vec<String>, // List of answers
}

#[derive(Deserialize)]
pub struct AskMeContent {
    pub title: String,            // Question title
    pub subtitle: String,         // Question subtitle
    pub shuffle: bool,            // Shuffle questions ordering?
    pub loop_questions: bool,     // Loop questions?
    pub case_sensitive: bool,     // Use case sensitive comparison?
    pub show_correct: bool,       // Show correct answer if user incorrectly answered the question?
    pub wait_duration: f64,       // Duration to wait before switching to the next question
    pub questions: Vec<Question>, // List of questions
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
    sleep(Duration::from_secs_f64(secs));
}
/** Print human-readable error to user */
pub fn raise_user_err(message: &str) {
    println!("{}", format!("{}{}", "ERROR: ", message).red());
    exit(1);
}

/** Get user input with prompt */
pub fn read_user_input() -> String {
    loop {
        print!("> ");
        let mut user_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        if !user_input.trim().is_empty() {
            break user_input;
        }
    }
}
pub struct App<'a> {
    pub askme_content: &'a mut AskMeContent,
    pub q_index: usize,
    pub correct_count: i32,
    pub rng: &'a mut ThreadRng,
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
        println!(" {}\n", self.askme_content.subtitle.blue());
    }

    pub fn print_question(&self) {
        println!("{}", self.askme_content.questions[self.q_index].title);
    }

    pub fn check_for_empty_questions(&self) -> bool {
        self.askme_content.questions.is_empty()
    }

    fn check_answer(&mut self, user_answer: String) {
        let mut tmp_is_correct = false;
        for answer in &self.askme_content.questions[self.q_index].answers {
            let tmp_answer = &mut String::new();
            if !self.askme_content.case_sensitive {
                *tmp_answer = answer.to_lowercase();
            } else {
                *tmp_answer = (&answer).to_string();
            }

            if *user_answer.trim() == *tmp_answer {
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

    fn index_change_loop(&mut self) {
        /* If shuffle is enabled, we get a random index from our questions
        vector instead of wrapping back to 0 when there are no questions left. */
        if self.askme_content.shuffle {
            self.q_index = self.rng.gen_range(0..self.askme_content.questions.len())
        }
        if self.q_index == self.askme_content.questions.len() {
            self.q_index = 0; // Go back to index 0 if we are looping
        }
    }

    fn delay_question(&self) {
        wait_for(self.askme_content.wait_duration);
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

        /* Check answer */
        self.check_answer(user_answer);

        /* Increment index */
        self.q_index += 1;

        /* When are looping, either wrap back to index 0 or get a random index */
        if self.askme_content.loop_questions {
            self.index_change_loop();
        }

        self.delay_question();
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
            if self.askme_content.shuffle {
                /* Shuffle the vector first */
                self.askme_content.questions = shuffle_arr(&self.askme_content.questions);
            }
            while self.q_index < self.askme_content.questions.len() {
                self.ask_question_routine();
            }
        }

        self.print_correct();
    }
}
