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
use askme::print_correct_answers;

mod app;

use app::App;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,

    #[arg(
        default_value_t = 1.0,
        long = "duration",
        short = 'd',
        help = "Choose how long the delay is between new questions (seconds, can be decimal)"
    )]
    wait_duration: f64,

    #[arg(
        long,
        short = 'L',
        help = "Choose if you would like to be quizzed in a loop"
    )]
    loop_questions: bool,

    #[arg(
        long,
        short = 'C',
        help = "Maximum choices for each question",
        default_value_t = 4
    )]
    max_choices: usize,

    #[arg(
        long,
        short = 'S',
        help = "show the correct answer(s) if an answer was wrong"
    )]
    show_correct: bool,
}

impl From<Args> for app::Settings {
    fn from(val: Args) -> Self {
        app::Settings {
            loop_questions: val.loop_questions,
            show_correct: val.show_correct,
            wait_duration: val.wait_duration,
            max_choices: val.max_choices,
        }
    }
}

fn main() {
    let args = Args::parse();

    let set = match AskmeSet::from_file(&args.filename) {
        Ok(s) => s,
        Err(e) => {
            askme::print_err(&format!("{}", e));
            std::process::exit(1)
        }
    };

    let set_questions = set.questions.len();

    let mut app = App::new(set, app::Settings::from(args));

    let correct_count = match app.run() {
        Ok(c) => c,
        Err(e) => panic!("error: {}", e),
    };

    print_correct_answers(correct_count, set_questions);
}
