/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetadataModelSigner {
    uri: String,
    #[serde(default = "Utc::now")]
    created: DateTime<Utc>,
}

#[allow(unused)]
impl MetadataModelSigner {
    pub fn new(uri: &str, created: DateTime<Utc>) -> Self {
        Self {
            uri: uri.to_string(),
            created,
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }
}
