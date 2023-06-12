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

use askme::{get_input, get_yn_from_input, prelude::*};
use colored::Colorize;
use figlet_rs::FIGfont;
use rand::Rng;

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

pub struct Settings {
    pub shuffle: bool,
    pub wait_duration: f64,
    pub loop_questions: bool,
}

pub struct App {
    set: AskmeSet,
    settings: Settings,
    correct_count: i32,
}

impl App {
    fn get_random_answer_from_set(&self, exclude_question_ans: Option<&Question>) -> &String {
        let answers = &self
            .set
            .questions
            .iter()
            .filter(|qn| {
                if let Some(q) = exclude_question_ans {
                    qn.answers != q.answers
                } else {
                    true
                }
            })
            .flat_map(|qn| &qn.answers)
            .collect::<Vec<&String>>();

        let rand_ans_idx = rand::thread_rng().gen_range(0..answers.len());

        answers[rand_ans_idx]
    }

    fn provide_qn_feedback(&self, question: &Question, is_correct: bool) {
        match is_correct {
            true => {
                println!("{}\n", CORRECT_FEEDBACK_STR.green().bold());
            }
            false => {
                println!("{}\n", INCORRECT_FEEDBACK_STR.red().bold());

                if self.settings.show_correct {
                    if question.answers.len() == 1 {
                        println!(
                            "the answer to \"{}\" is \"{}\"!",
                            question.title,
                            question.answers.first().unwrap()
                        );
                    } else {
                        let all_correct_answers_string = question.answers.join(", ");
                        println!(
                            "the answers to \"{}\" are \"{}\"!",
                            question.title, all_correct_answers_string
                        );
                    }
                }
            }
        }
    }

    fn ask_question(&mut self, question: &Question) {
        let rand_ans = self.get_random_answer_from_set(Some(question));
        println!(
            "{} (y/n)\n \"{} is {}\"\n",
            "Is this correct?".bold(),
            question.title,
            rand_ans
        );

        let user_answer = get_yn_from_input(get_input().trim());
        let is_rand_ans_correct = question.answers.contains(rand_ans) == user_answer;

        if is_rand_ans_correct {
            self.correct_count += 1;
        }

        self.provide_qn_feedback(question, is_rand_ans_correct);
    }
}

impl AskmeMode<Settings, i32> for App {
    fn new(set: AskmeSet, settings: Settings) -> Self {
        App {
            set,
            settings,
            correct_count: 0,
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
