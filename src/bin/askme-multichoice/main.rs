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
use colored::Colorize;
use figlet_rs::FIGfont;
use rand::rngs::ThreadRng;

static MCQ_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct App {
    set: AskmeSet,
    settings: AskmeSettings,
    correct_count: i32,
    rng: ThreadRng,
}

impl App {
    pub fn check_for_empty_questions(&self) -> bool {
        self.set.questions.is_empty()
    }

    pub fn print_question_choices(&self, question: &Question) {
        let columns = 2;
        let mut answers = question.answers.clone();

        let mut _offset = 0;

        while !answers.is_empty() {
            let mut curr_col: Vec<String> = Vec::new();
            for _ in 0..columns {
                if let Some(curr_item) = answers.pop() {
                    curr_col.push(curr_item)
                }
            }

            let curr_col_len = curr_col.len();

            let curr_col_string = curr_col
                .into_iter()
                .enumerate()
                .map(|(ans_idx, curr_ans)| format!("{}. {}", MCQ_LETTERS[ans_idx], curr_ans))
                .collect::<Vec<String>>()
                .join("    ");

            println!("{}", curr_col_string);

            _offset += curr_col_len;
        }
    }

    pub fn ask_question(&mut self, question: &Question) {
        println!("{}", question.title);
    }

    pub fn run_set(&mut self) {
        let mut qns = self.set.questions.clone();

        if self.settings.loop_questions {
            qns = shuffle_arr(&qns);
        }

        for question in qns.iter() {
            self.ask_question(question)
        }
    }
}

impl AskmeRunnable<i32> for App {
    fn new(set: AskmeSet, settings: AskmeSettings) -> Self {
        App {
            correct_count: 0,
            rng: rand::thread_rng(),
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

    fn run(&mut self) -> Result<i32, &str> {
        if self.check_for_empty_questions() {
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

fn main() {
    todo!()
}
