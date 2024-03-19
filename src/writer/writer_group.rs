/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::{super::Owner, GroupType};
use std::error::Error;

#[derive(Debug)]
pub struct WriterGroup {
    typ: GroupType,
    id: String,
}

#[allow(unused)]
impl WriterGroup {
    pub fn new(group: &str) -> Result<Self, Box<dyn Error>> {
        let split = group.split_once(':').unwrap_or((group, ""));
        let typ = match split.0 {
            "init" => GroupType::Initialize,
            "txn" => GroupType::Transaction,
            _ => return Err("invalid group type")?,
        };
        Ok(Self {
            typ,
            id: split.1.to_string(),
        })
    }

    pub fn new_txn(owner: &Owner) -> Result<Self, Box<dyn Error>> {
        let provider = owner.provider().clone().ok_or("No provider")?;
        match owner.address() {
            Some(address) => Ok(Self {
                typ: GroupType::Transaction,
                id: format!("{}:{}", provider, address),
            }),
            None => Ok(Self {
                typ: GroupType::Transaction,
                id: format!("{}", provider),
            }),
        }
    }

    pub fn new_init(owner: &Owner) -> Result<Self, Box<dyn Error>> {
        let provider = owner.provider().clone().ok_or("No provider")?;
        Ok(Self {
            typ: GroupType::Initialize,
            id: format!("{}", provider),
        })
    }

    pub fn typ(&self) -> &GroupType {
        &self.typ
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn to_string(&self) -> String {
        match self.typ {
            GroupType::Initialize => format!("init:{}", self.id),
            GroupType::Transaction => format!("txn:{}", self.id),
        }
    }
}
