use askme_memorize::App;
use clap::Parser;
mod yaml_parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// AskMe file
    #[arg()]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let askme_content = yaml_parser::parse_file(&args.filename);
    let mut app = App {
        askme_content: &askme_content,
        q_index: 0,
        correct_count: 0,
    };

    app.main_loop();
}
