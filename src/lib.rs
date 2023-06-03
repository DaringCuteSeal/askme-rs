use colored::*;
use figlet_rs::FIGfont;
use rand::{rngs::ThreadRng, Rng};
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

#[derive(Deserialize, Clone)]
pub struct Question {
    pub title: String,        // Question title
    pub answers: Vec<String>, // List of answers
}

pub struct Settings {
    pub shuffle: bool,
    pub loop_questions: bool,
    pub case_sensitive: bool,
    pub show_correct: bool,
    pub wait_duration: f64,
}

#[derive(Deserialize)]
pub struct AskmeFile {
    pub title: String,            // Question title
    pub subtitle: String,         // Question subtitle
    pub questions: Vec<Question>, // List of questions
}

pub fn parse_file(filename: &str) -> Result<AskmeFile, String> {
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
        stdout.write_all("> ".as_bytes()).expect("you have just encountered an ultra-rare error: failed to write to the standard output!"); // very ugly
        stdout.flush().expect("you have just encountered an ultra-rare error: failed to flush the standard output after writing to it!");
        // yes i know

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

pub struct App {
    pub askme_file: AskmeFile,
    pub settings: Settings,
    pub correct_count: i32,
    pub rng: ThreadRng,
}

impl App {
    pub fn from_file(file_name: &str, settings: Settings) -> Result<Self, String> {
        let content = match parse_file(file_name) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let app = App {
            askme_file: content,
            correct_count: 0,
            rng: rand::thread_rng(),
            settings,
        };

        Ok(app)
    }

    pub fn print_title(&self) {
        let font = FIGfont::standard().unwrap();
        let title_text = font
            .convert(&self.askme_file.title)
            .unwrap()
            .to_string()
            .cyan();

        println!("{}", title_text);
    }

    pub fn print_subtitle(&self) {
        println!(" {}\n", self.askme_file.subtitle.blue());
    }

    pub fn print_question(&self, question: &Question) {
        println!("{}", question.title);
    }

    pub fn check_for_empty_questions(&self) -> bool {
        self.askme_file.questions.is_empty()
    }

    fn check_answer(&self, question: &Question, user_answer: String) -> bool {
        match self.settings.case_sensitive {
            true => question.answers.contains(&user_answer.trim().to_string()),
            false => {
                let questions_lowercase = &question
                    .answers
                    .iter()
                    .map(|a| a.to_lowercase())
                    .collect::<Vec<String>>();

                questions_lowercase.contains(&user_answer.trim().to_string())
            }
        } // keep it around first
    }

    pub fn print_question_answers(&self, question: &Question) {
        let answers = &question.answers;

        if answers.is_empty() {
            println!("The correct answer is: {}\n", answers[0].bold());
            return;
        }

        let ans_text = answers.join(", ");
        println!("{}{}", "The Correct answers are: ".bold(), ans_text);
    }

    pub fn provide_qn_feedback(&self, question: &Question, correct: bool) {
        match correct {
            true => println!("{}\n", CORRECT_FEEDBACK_STR.green()),
            false => {
                println!("{}\n", INCORRECT_FEEDBACK_STR.red());

                if self.settings.show_correct {
                    self.print_question_answers(question);
                }
            }
        };
    }

    pub fn ask_question(&mut self, question: &Question) {
        self.print_question(question);

        let user_answer = match self.settings.case_sensitive {
            true => read_user_input(),
            false => read_user_input().to_lowercase(),
        };

        let correct = self.check_answer(question, user_answer);
        self.provide_qn_feedback(question, correct);

        match correct {
            true => self.correct_count += 1,
            false => (),
        };

        wait_for(self.settings.wait_duration);
    }

    fn run_set(&mut self) {
        let mut qns = self.askme_file.questions.clone();

        if self.settings.shuffle {
            qns = shuffle_arr(&qns);
        }

        for question in qns.iter() {
            self.ask_question(question)
        }
    }

    pub fn run(&mut self) -> Result<i32, &str> {
        if self.check_for_empty_questions() {
            return Err("no questions provided!");
        };

        self.print_title();
        self.print_subtitle();

        match self.settings.loop_questions {
            true => loop {
                self.run_set()
            },
            false => self.run_set(),
        }

        Ok(self.correct_count)
    }
}
