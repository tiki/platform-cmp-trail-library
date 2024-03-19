/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::super::{utils::byte_helpers, Transaction};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WriterBodyInitialize {
    #[serde(default = "Utc::now")]
    timestamp: DateTime<Utc>,
    #[serde(default)]
    key: String,
}

impl WriterBodyInitialize {
    pub fn new(timestamp: DateTime<Utc>, key: &str) -> Self {
        Self {
            timestamp,
            key: key.to_string(),
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}
