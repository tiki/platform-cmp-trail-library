/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    provider: Option<String>,
    address: Option<String>,
}

impl Owner {
    pub fn new(provider: Option<String>, address: Option<String>) -> Self {
        Self { provider, address }
    }
    pub fn provider(&self) -> &Option<String> {
        &self.provider
    }
    pub fn address(&self) -> &Option<String> {
        &self.address
    }
}
