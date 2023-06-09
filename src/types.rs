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

use serde::Deserialize;
use std::fs;

use crate::prelude::AskmeError;
use crate::prelude::FromSetFile;

#[derive(Deserialize, Clone)]
pub struct Question {
    pub title: String,        // Question title
    pub answers: Vec<String>, // List of answers
}

#[derive(Deserialize)]
pub struct AskmeSet {
    pub title: String,            // Question title
    pub subtitle: String,         // Question subtitle
    pub questions: Vec<Question>, // List of questions
}

impl FromSetFile for AskmeSet {
    fn from_file(file_name: &str) -> Result<AskmeSet, AskmeError> {
        let yaml_file = match fs::read_to_string(file_name) {
            Ok(file) => file,
            Err(e) => {
                return Err(AskmeError::new(
                    format!("Failed to read file '{}': {}", file_name, e),
                    Some(Box::from(e)),
                ))
            }
        };

        let content: AskmeSet = match serde_yaml::from_str(&yaml_file) {
            Ok(file_struct) => file_struct,
            Err(e) => {
                return Err(AskmeError::new(
                    format!("Failed to parse file content: {}", e),
                    Some(Box::from(e)),
                ))
            }
        };

        Ok(content)
    }
}
