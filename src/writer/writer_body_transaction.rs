/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::super::{utils::byte_helpers, Transaction};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WriterBodyTransaction {
    transactions: Vec<String>,
}

impl WriterBodyTransaction {
    pub fn default() -> Self {
        WriterBodyTransaction {
            transactions: vec![],
        }
    }

    pub fn new(transactions: &Vec<Vec<u8>>) -> Self {
        WriterBodyTransaction {
            transactions: transactions
                .iter()
                .map(|txn| byte_helpers::base64_encode(txn))
                .collect(),
        }
    }

    pub fn add_transaction(mut self, transaction: &Vec<u8>) -> Self {
        self.transactions
            .push(byte_helpers::base64_encode(transaction));
        self
    }

    pub fn transactions(&self) -> Result<Vec<Transaction>, Box<dyn Error>> {
        let mut transactions = vec![];
        for transaction in &self.transactions {
            let bytes = byte_helpers::base64_decode(transaction)?;
            let transaction = Transaction::deserialize(&bytes)?;
            transactions.push(transaction);
        }
        Ok(transactions)
    }
}
