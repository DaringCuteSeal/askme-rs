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

use askme::prelude::*;
use askme::{get_input, shuffle_arr, wait_for};
use colored::Colorize;
use figlet_rs::FIGfont;

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

pub struct Settings {
    pub shuffle: bool,
    pub loop_questions: bool,
    pub case_sensitive: bool,
    pub show_correct: bool,
    pub wait_duration: f64,
}

pub struct App {
    set: AskmeSet,
    settings: Settings,
    correct_count: i32,
}

impl App {
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
        }
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

    fn provide_qn_feedback(&self, question: &Question, correct: bool) {
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

    fn ask_question(&mut self, question: &Question) {
        println!(" {}", question.title.bold());

        let user_answer = match self.settings.case_sensitive {
            true => get_input(),
            false => get_input().to_lowercase(),
        };

        let correct = self.check_answer(question, user_answer);
        self.provide_qn_feedback(question, correct);

        match correct {
            true => self.correct_count += 1,
            false => (),
        };

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
        font.convert(&self.set.title)
            .unwrap()
            .to_string()
            .cyan()
            .to_string()
    }

    fn get_subtitle(&self) -> String {
        format!(" {}\n", self.set.subtitle.blue())
    }

    fn run_set(&mut self) {
        let qns = match self.settings.shuffle {
            false => self.set.questions.clone(),
            true => shuffle_arr(&self.set.questions.clone()),
        };

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
