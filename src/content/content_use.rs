/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::UseCase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContentUse {
    #[serde(alias = "usecases")]
    use_cases: Vec<UseCase>,
    destinations: Option<Vec<String>>,
}

impl ContentUse {
    pub fn use_cases(&self) -> &Vec<UseCase> {
        &self.use_cases
    }

    pub fn destinations(&self) -> &Option<Vec<String>> {
        &self.destinations
    }
}
