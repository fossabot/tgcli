use std::fmt::Display;

use super::SendOperation;

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

pub struct MessageParams {
    message: String,
}

impl MessageParams {
    pub fn new(message: String) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub struct SendMessageOperation {
    params: MessageParams,
}

impl SendMessageOperation {
    pub fn new(params: MessageParams) -> Self {
        Self { params }
    }
}

impl SendOperation for SendMessageOperation {
    fn send(self) -> Result<(), crate::operations::OperationError> {
        todo!() // TODO implement send message operation
    }
}
