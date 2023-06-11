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

        let correct_answer_location = rng.gen_range(0..len - 1);

        let curr_question_correct_answer_location = rng.gen_range(0..curr_question.answers.len());

        wrong_answers.insert(
            correct_answer_location,
            curr_question.answers[curr_question_correct_answer_location].clone(),
        );

        // now wrong_answers now contains the correct answer
        (wrong_answers, correct_answer_location)
    }

    pub fn get_question_choices(&self, list_of_answers: &[String]) -> String {
        let columns = 2;
        let mut retval = String::new();
        let mut answers = list_of_answers.to_owned();
        answers.reverse();

        let mut _offset = 0;

        while !answers.is_empty() {
            let mut current_column: Vec<String> = Vec::new();

            // no slicing and copying because space complexity
            for _ in 0..columns {
                if let Some(curr_item) = answers.pop() {
                    current_column.push(curr_item)
                }
            }

            // prevent the need to use a moved value later on
            let curr_col_len = current_column.len();

            let curr_col_string = current_column
                .into_iter()
                .enumerate()
                .map(|(ans_idx, curr_ans)| {
                    // add the index and offset.
                    //
                    // the index does begin from 0, but
                    // so does the characters, so no need
                    // to add one.
                    format!("{}. {}", MCQ_LETTERS[ans_idx + _offset], curr_ans)
                })
                .collect::<Vec<String>>()
                .join("    "); // space things out a bit; TODO: make the spacing actually a bit more consistent

            retval += &format!("{}\n", curr_col_string.as_str());

            // offset the index of the letters.
            //
            // say if a, b, c was used in one column,
            // 3 would be added to the offset so the
            // next row would be d, e, f.
            //
            _offset += curr_col_len;
        }

        retval
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

        for question in qns {
            self.ask_question(&question)
        }
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

impl AskmeCliMode for App {
    fn provide_qn_feedback(&self, correct: bool, correct_choice_index: usize) {
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

    fn ask_question(&mut self, question: &Question) {
        println!("{}", question.title);

        let available_answers =
            self.aggregate_answers(question, &self.set.questions, self.settings.max_choices);

        println!("{}", self.get_question_choices(&available_answers.0));

        let user_answer_idx = loop {
            let input = askme::get_input().to_ascii_lowercase();
            let first_char = input.chars().collect::<Vec<char>>()[0];

            let ans_idx = MCQ_LETTERS.binary_search(&first_char).unwrap_or(0);

            let valid_chars = (0..4).map(|i| MCQ_LETTERS[i]).collect::<Vec<char>>();

            if valid_chars.contains(&first_char) {
                break ans_idx;
            }
        };

        let is_correct =
            available_answers.0[available_answers.1] == available_answers.0[user_answer_idx];

        self.provide_qn_feedback(is_correct, available_answers.1);
        wait_for(self.settings.wait_duration);
    }
}
