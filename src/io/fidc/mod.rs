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

//! A module for file ID caching.

use sqlx::SqliteConnection;

use crate::operations::OperationError;

/// Types of connections for file ID caching.
enum DbConnection {
    Sqlite(SqliteConnection),
}

#[async_trait]
/// A trait for file id-hash caching.
trait FileCache {
    /// Initializes the database. It must be used before everything else.
    async fn initialize(&self) -> Result<(), OperationError>;
    /// Creates or updates current hash and file id.
    async fn upsert_id(&self, hash: String, fileid: String) -> Result<(), OperationError>;
    /// Gets the file id for the hash.
    async fn get_id(&self, hash: String) -> Option<String>;
}
