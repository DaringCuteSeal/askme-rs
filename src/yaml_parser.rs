use askme_memorize::{raise_user_err, AskMeContent};
use std::fs::File;
use std::io::Read;

// Crappy error handling
// Finish later .-.
pub fn parse_file(filename: &String) -> AskMeContent {
    let mut file = File::open(filename).expect("Failed to open file!");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    let askme_file = serde_yaml::from_str(&file_content);
    let askme_file: AskMeContent = match askme_file {
        Ok(file_struct) => file_struct,
        Err(error) => {
            raise_user_err(&format!(
                "Failed to parse file!\nError message: {:?}",
                error
            ));
            panic!()
        }
    };
    askme_file
}
