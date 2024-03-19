/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

pub trait ContentSerializer {
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn deserialize(bytes: &Vec<u8>) -> Result<Box<Self>, Box<dyn Error>>;
}
