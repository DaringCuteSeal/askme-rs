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
        help = "Make answers to the questions case-sensitive"
    )]
    case_sensitive: bool,

    #[arg(
        long,
        short = 'S',
        help = "show the correct answer(s) if an answer was wrong"
    )]
    show_correct: bool,
}

impl From<Args> for app::AskmeSettings {
    fn from(val: Args) -> Self {
        app::AskmeSettings {
            loop_questions: val.loop_questions,
            case_sensitive: val.case_sensitive,
            show_correct: val.show_correct,
            wait_duration: val.wait_duration,
        }
    }
}

fn main() {
    let args = Args::parse();

    let set = match AskmeSet::from_file(&args.filename) {
        Ok(s) => s,
        Err(e) => panic!("error: {}", e),
    };

    let set_questions = set.questions.len();

    let mut app = App::new(set, app::AskmeSettings::from(args));

    let correct_count = match app.run() {
        Ok(c) => c,
        Err(e) => panic!("error: {}", e),
    };

    print_correct_answers(correct_count, set_questions);
}
