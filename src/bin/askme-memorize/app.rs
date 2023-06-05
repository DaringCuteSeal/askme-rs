use askme::*; // fuck i have to fix this shit
use colored::Colorize;
use figlet_rs::FIGfont;
use rand::rngs::ThreadRng;

const CORRECT_FEEDBACK_STR: &str = "✔️ That's correct!";
const INCORRECT_FEEDBACK_STR: &str = "❌ Not quite correct..";

pub struct App {
    pub askme_file: AskmeSet,
    pub settings: AskmeSettings,
    pub correct_count: i32,
    pub rng: ThreadRng,
}

impl App {
    pub fn from_file(file_name: &str, settings: AskmeSettings) -> Result<Self, String> {
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
