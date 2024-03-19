/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::{
    super::utils::{byte_helpers, compact_size},
    Serializer, Tag,
};
use std::error::Error;

pub struct ContentEmpty {}

impl ContentEmpty {
    pub fn new() -> Self {
        Self {}
    }
}

impl Serializer for ContentEmpty {
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(vec![0])
    }

    fn deserialize(bytes: &Vec<u8>) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(Self {}))
    }
}
