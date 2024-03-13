/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::{ ModelSigner, super::Owner};

pub fn current_version() -> i32 { 1 }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(default = "current_version")]
    pub version: i32,
    pub owner: Owner,
    pub last_block: String,
    pub blocks: Vec<String>,
    pub signers: Vec<ModelSigner>,
    #[serde(default = "Utc::now")]
    pub modified: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub created: DateTime<Utc>
}

#[allow(unused)]
impl Model {
    pub fn version(&self) -> i32 { self.version }
    pub fn owner(&self) -> &Owner { &self.owner }
    pub fn last_block(&self) -> &str { &self.last_block }
    pub fn blocks(&self) -> &Vec<String> { &self.blocks }
    pub fn signers(&self) -> &Vec<ModelSigner> { &self.signers }
    pub fn modified(&self) -> DateTime<Utc> { self.modified }
    pub fn created(&self) -> DateTime<Utc> { self.created }
}
