use std::convert::TryFrom;

use futures::executor;
use reqwest::multipart::Form;

use crate::{
    http::request::models::generate_form_part_from_file,
    operations::{
        bot::send::{self, document::SendDocumentParams},
        OperationError,
    },
};

use super::{ChatId, InputFile, ParseMode};

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

#[derive(Debug)]
/// A model for /sendDocument request.
pub struct SendDocumentRequestModel {
    chat_id: ChatId,
    document: InputFile,
    thumbnail: Option<InputFile>,
    caption: Option<String>,
    parse_mode: ParseMode,
    disable_notification: bool,
}

impl TryFrom<SendDocumentRequestModel> for Form {
    type Error = OperationError;

    fn try_from(m: SendDocumentRequestModel) -> Result<Self, Self::Error> {
        debug!("Converting SendDocumentRequestModel to Form...");
        let chat_id = m.chat_id.to_string();
        let parse_mode = m.parse_mode.to_string();

        let initial_form = Form::new()
            .text("chat_id", chat_id)
            .text("parse_mode", parse_mode);

        let caption_form = match m.caption {
            Some(c) => initial_form.text("caption", c),
            None => initial_form,
        };

        let document_form = match m.document {
            InputFile::Local(p) => match executor::block_on(generate_form_part_from_file(p)) {
                Ok(part) => caption_form.part("document", part),
                Err(e) => return Err(e),
            },
            InputFile::Remote(u) => caption_form.text("document", u.to_string()),
            InputFile::Id(i) => caption_form.text("document", i),
        };

        let thumbnail_form = match m.thumbnail {
            Some(inputfile) => match inputfile {
                InputFile::Local(p) => match executor::block_on(generate_form_part_from_file(p)) {
                    Ok(part) => document_form.part("thumbnail", part),
                    Err(e) => return Err(e),
                },
                InputFile::Remote(u) => document_form.text("thumbnail", u.to_string()),
                InputFile::Id(i) => document_form.text("thumbnail", i),
            },
            None => document_form,
        };

        let notification_form = match m.disable_notification {
            true => thumbnail_form.text("disable_notification", "true"),
            false => thumbnail_form,
        };

        Ok(notification_form)
    }
}

impl From<SendDocumentParams> for SendDocumentRequestModel {
    fn from(params: SendDocumentParams) -> Self {
        debug!("Converting SendDocumentParams to SendDocumentRequestModel...");

        let chat_id = match params.2.receiver.parse::<usize>() {
            Ok(v) => ChatId::Int(v),
            Err(_) => ChatId::Str(params.2.receiver),
        };

        let caption = params.3.message;

        let parse_mode = match params.2.format {
            send::MessageFormat::Markdown => ParseMode::Markdown,
            send::MessageFormat::HTML => ParseMode::HTML,
        };

        let document = InputFile::Local(params.3.file);

        let thumbnail = match params.3.thumbnail {
            Some(p) => Some(InputFile::Local(p)),
            None => None,
        };

        let disable_notification = params.2.silent;

        SendDocumentRequestModel {
            chat_id,
            caption,
            parse_mode,
            document,
            thumbnail,
            disable_notification,
        }
    }
}
