mod app;

use app::App;
use askme::{print_correct_answers, AskmeSettings};
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

    #[arg(long, short = 's', help = "Enable shuffling")]
    shuffle: bool,

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

impl From<Args> for AskmeSettings {
    fn from(val: Args) -> Self {
        AskmeSettings {
            shuffle: val.shuffle,
            loop_questions: val.loop_questions,
            case_sensitive: val.case_sensitive,
            show_correct: val.show_correct,
            wait_duration: val.wait_duration,
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut app = match App::from_file(&args.filename.clone(), args.into()) {
        Ok(a) => a,
        Err(e) => panic!("error: {}", e),
    };

    let correct_count = match app.run() {
        Ok(c) => c,
        Err(e) => panic!("error: {}", e),
    };

    print_correct_answers(correct_count, app.askme_file.questions.len());
}
