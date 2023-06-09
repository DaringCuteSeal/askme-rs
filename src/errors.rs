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

/// Error definitions.
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AskmeError {
    pub cause: Option<Box<dyn Error>>,
    description: String,
}

impl AskmeError {
    pub fn new<S: AsRef<str>>(description: S, cause: Option<Box<dyn Error>>) -> Self {
        AskmeError {
            cause,
            description: description.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for AskmeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for AskmeError {
    fn cause(&self) -> Option<&dyn Error> {
        if let Some(e) = &self.cause {
            Some(&**e)
        } else {
            None
        }
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
