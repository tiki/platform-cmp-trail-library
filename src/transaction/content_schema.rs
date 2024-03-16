/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use num_bigint::BigInt;
use super::{ContentType, super::utils::{byte_helpers, compact_size}};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContentSchema {
    typ: ContentType,
    schema: u16
}

impl ContentSchema {
    pub fn new(schema: u16) -> Result<Self, Box<dyn Error>> {
        match schema {
            2 => Ok(Self { typ: ContentType::Title, schema: 2 }),
            3 => Ok(Self { typ: ContentType::License, schema: 3 }),
            4 => Ok(Self { typ: ContentType::Payable, schema: 4 }),
            5 => Ok(Self { typ: ContentType::Receipt, schema: 5 }),
            _ => Err("Unknown schema".into())
        }
    }

    pub fn title() -> Self { Self { typ: ContentType::Title, schema: 2 } }
    pub fn license() -> Self { Self { typ: ContentType::License, schema: 3 } }
    pub fn payable() -> Self { Self { typ: ContentType::Payable, schema: 4 } }
    pub fn receipt() -> Self { Self { typ: ContentType::Receipt, schema: 5 } }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        let schema_bigint = &BigInt::from(self.schema);
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(schema_bigint)));
        Ok(bytes)
    }

    pub fn deserialize(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let schema = byte_helpers::decode_bigint(bytes);
        let schema = schema.to_string().parse::<u16>()?;
        let schema = Self::new(schema)?;
        Ok(schema)
    }

    pub fn typ(&self) -> &ContentType { &self.typ }
    pub fn schema(&self) -> u16 { self.schema }
}