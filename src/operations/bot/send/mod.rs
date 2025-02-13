use crate::operations::OperationError;

// Copyright 2021 Eray Erdin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod audio;
pub mod document;
pub mod location;
pub mod message;
pub mod photo;
pub mod poll;
pub mod video;

#[derive(Debug)]
pub enum MessageFormat {
    Markdown,
    HTML,
}

#[derive(Debug)]
pub struct SendParams {
    pub receiver: String,
    pub format: MessageFormat,
    pub silent: bool,
}

impl SendParams {
    pub fn new(receiver: &str, format: MessageFormat, silent: bool) -> Self {
        Self {
            receiver: String::from(receiver),
            format,
            silent,
        }
    }
}

#[async_trait]
pub trait SendOperation {
    fn send(self) -> Result<(), OperationError>;
}
