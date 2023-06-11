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

impl App {}

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
        let mut qns = self.set.questions.clone();

        if self.settings.shuffle {
            qns = shuffle_arr(&qns);
        }

        for question in qns {
            self.ask_question(&question)
        }
    }

    fn run(&mut self) -> Result<i32, &str> {
        if self.set.questions.is_empty() {
            return Err("no questions provided!");
        }

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

impl AskmeCliMode for App {
    fn ask_question(&mut self, question: &Question) {
        println!("{}", question.title);

        // let user_answer = match self.settings.
    }
    fn provide_qn_feedback(&self, question: &Question, correct: bool) {}
}
