use std::path;

use directories::ProjectDirs;

use crate::operations::{CommonExitCodes, OperationError};

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

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "erayerdin";
const APPLICATION: &str = "tgcli";

async fn get_project_dirs() -> Result<ProjectDirs, OperationError> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(p) => Ok(p),
        None => Err(OperationError::new(
            CommonExitCodes::DirectoriesError as i32,
            "Could not get application directories.",
        )),
    }
}

async fn get_cache_dir() -> Result<path::PathBuf, OperationError> {
    debug!("Getting cache directory...");

    let project_dirs = match get_project_dirs().await {
        Ok(p) => p,
        Err(_) => {
            return Err(OperationError::new(
                CommonExitCodes::DirectoriesCacheDirError as i32,
                "Could not get project cache directory.",
            ))
        }
    };

    Ok(project_dirs.cache_dir().to_path_buf())
}
