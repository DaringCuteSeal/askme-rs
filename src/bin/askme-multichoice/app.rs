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

use askme::{prelude::*, print_warning, wait_for};
use colored::Colorize;
use figlet_rs::FIGfont;
use inquire::{InquireError, Select};
use rand::Rng;

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

static MCQ_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct Settings {
    pub loop_questions: bool,
    pub show_correct: bool,
    pub wait_duration: f64,
    pub max_choices: usize,
}

pub struct App {
    set: AskmeSet,
    settings: Settings,
    correct_count: i32,
}

impl App {
    /// Randomly generate a list of answers based on the set answers.
    pub fn aggregate_answers(
        &self,
        curr_question: &Question,
        questions: &[Question],
        mut len: usize,
    ) -> (Vec<String>, usize) {
        let mut choices_len = 0;
        for ele in questions {
            choices_len += ele.answers.len();
        }

        if choices_len < len {
            print_warning(&format!(
                "Not enough answers to aggregate! Reducing maximum choices to {}",
                choices_len
            ));
            len = choices_len
        }

        let mut rng = rand::thread_rng();

        // Tuple of question idx and answer idx
        let mut added_index = Vec::new();

        let correct_answer = &curr_question.answers[rng.gen_range(0..curr_question.answers.len())];

        let mut wrong_answers = (0..len - 1)
            .map(|_| loop {
                let idx = rng.gen_range(0..questions.len());
                let answers_len = questions[idx].answers.len();
                let answers_idx = rng.gen_range(0..answers_len);
                if !(added_index.contains(&(idx, answers_idx))
                    || self.set.questions[idx].answers[answers_idx] == *correct_answer)
                {
                    added_index.push((idx, answers_idx));
                    break questions[idx].answers[answers_idx].clone();
                }
            })
            .collect::<Vec<String>>();

        let correct_answer_location = rng.gen_range(0..len);

        let curr_question_correct_answer_location = rng.gen_range(0..curr_question.answers.len());

        wrong_answers.insert(
            correct_answer_location,
            curr_question.answers[curr_question_correct_answer_location].clone(),
        );

        // now wrong_answers now contains the correct answer
        (wrong_answers, correct_answer_location)
    }

    pub fn provide_qn_feedback(&self, correct: bool, correct_choice_index: usize) {
        match correct {
            true => println!("{}\n", CORRECT_FEEDBACK_STR.green()),
            false => {
                println!("{}\n", INCORRECT_FEEDBACK_STR.red());

                if self.settings.show_correct {
                    println!(
                        "{}",
                        format!(
                            "The correct option is: {}",
                            MCQ_LETTERS[correct_choice_index].to_string().bold() // place the letter inside
                        )
                        .red()
                    )
                }
            }
        }
    }

    pub fn ask_question(&mut self, question: &Question) {
        println!(" {}", question.title.bold());

        let available_answers =
            self.aggregate_answers(question, &self.set.questions, self.settings.max_choices);

        let user_answer = match Select::new("Answer:", available_answers.0.clone()).prompt() {
            Ok(answer) => answer,
            Err(err) => {
                if let InquireError::OperationInterrupted = err {
                    std::process::exit(1)
                } else {
                    panic!("Failed reading from stdin!")
                }
            }
        };

        let is_correct = available_answers.0[available_answers.1] == user_answer;

        if is_correct {
            self.correct_count += 1;
        }

        self.provide_qn_feedback(is_correct, available_answers.1);
        wait_for(self.settings.wait_duration);
    }
}

impl AskmeMode<Settings, i32> for App {
    fn new(set: AskmeSet, settings: Settings) -> Self {
        App {
            correct_count: 0,
            set,
            settings,
        }
    }

    fn get_title(&self) -> String {
        let font = FIGfont::standard().unwrap();
        let title_text = font.convert(&self.set.title).unwrap().to_string().cyan();

        format!("{}", title_text)
    }

    fn get_subtitle(&self) -> String {
        format!(" {}\n", self.set.subtitle.blue())
    }

    fn run_set(&mut self) {
        let qns = self.set.questions.clone();

        qns.iter().for_each(|question| self.ask_question(question));
    }

    fn run(&mut self) -> Result<i32, &str> {
        if self.set.questions.is_empty() {
            return Err("no questions provided!");
        };

        println!("{}", self.get_title());
        println!("{}", self.get_subtitle());

        match self.settings.loop_questions {
            true => loop {
                self.run_set()
            },
            false => self.run_set(),
        }

        Ok(self.correct_count)
    }
}
