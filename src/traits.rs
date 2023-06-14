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

use crate::prelude::*;

pub trait AskmeMode<S, R> {
    fn get_title(&self) -> String;
    fn get_subtitle(&self) -> String;
    fn new(set: AskmeSet, settings: S) -> Self;
    fn run_set(&mut self);
    fn run(&mut self) -> Result<R, &str>;
}

pub trait FromSetFile {
    fn from_file(file_name: &str) -> Result<AskmeSet, AskmeError>;
}
