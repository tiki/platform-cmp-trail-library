/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::super::{transaction::Model as TxnModel};
use chrono::{DateTime, Utc};
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Model {
    pub id: String,
    pub version: i32,
    pub timestamp: DateTime<Utc>,
    pub previous_id: String,
    pub transaction_root: String,
    pub transactions: Vec<TxnModel>,
    pub bytes: Vec<u8>,
}

#[allow(unused)]
impl Model {

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn version(&self) -> i32 {
        self.version
    }
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    pub fn previous_id(&self) -> &str {
        &self.previous_id
    }
    pub fn transaction_root(&self) -> &str {
        &self.transaction_root
    }
    pub fn transactions(&self) -> &Vec<TxnModel> {
        &self.transactions
    }
}
