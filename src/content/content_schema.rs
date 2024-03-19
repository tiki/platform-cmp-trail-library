/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::{
    super::utils::{byte_helpers, compact_size},
    SchemaType,
};
use num_bigint::BigInt;
use std::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContentSchema {
    typ: SchemaType,
    schema: u16,
}

impl ContentSchema {
    pub fn new(schema: u16) -> Result<Self, Box<dyn Error>> {
        match schema {
            0 => Ok(Self {
                typ: SchemaType::Empty,
                schema: 2,
            }),
            2 => Ok(Self {
                typ: SchemaType::Title,
                schema: 2,
            }),
            3 => Ok(Self {
                typ: SchemaType::License,
                schema: 3,
            }),
            4 => Ok(Self {
                typ: SchemaType::Payable,
                schema: 4,
            }),
            5 => Ok(Self {
                typ: SchemaType::Receipt,
                schema: 5,
            }),
            _ => Err("Unknown schema".into()),
        }
    }

    pub fn empty() -> Self {
        Self {
            typ: SchemaType::Title,
            schema: 0,
        }
    }

    pub fn title() -> Self {
        Self {
            typ: SchemaType::Title,
            schema: 2,
        }
    }

    pub fn license() -> Self {
        Self {
            typ: SchemaType::License,
            schema: 3,
        }
    }

    pub fn payable() -> Self {
        Self {
            typ: SchemaType::Payable,
            schema: 4,
        }
    }

    pub fn receipt() -> Self {
        Self {
            typ: SchemaType::Receipt,
            schema: 5,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        let schema_bigint = &BigInt::from(self.schema);
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
            schema_bigint,
        )));
        Ok(bytes)
    }

    pub fn deserialize(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let schema = byte_helpers::decode_bigint(bytes);
        let schema = schema.to_string().parse::<u16>()?;
        let schema = Self::new(schema)?;
        Ok(schema)
    }

    pub fn typ(&self) -> &SchemaType {
        &self.typ
    }

    pub fn schema(&self) -> u16 {
        self.schema
    }
}
