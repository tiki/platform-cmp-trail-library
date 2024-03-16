/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::{
    super::utils::{byte_helpers, compact_size},
    Serializer, Tag,
};
use std::error::Error;

pub struct ContentTitle {
    ptr: String,
    origin: String,
    tags: Vec<Tag>,
    description: Option<String>,
}

impl ContentTitle {
    pub fn new(ptr: &str, origin: &str, tags: Vec<Tag>, description: Option<String>) -> Self {
        Self {
            ptr: ptr.to_string(),
            origin: origin.to_string(),
            tags,
            description,
        }
    }

    pub fn ptr(&self) -> &str {
        &self.ptr
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}

impl Serializer for ContentTitle {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        let tags = serde_json::to_string(&self.tags)?;
        bytes.append(&mut compact_size::encode(byte_helpers::utf8_encode(
            &self.ptr,
        )));
        bytes.append(&mut compact_size::encode(byte_helpers::utf8_encode(
            &self.origin,
        )));
        if self.description.is_some() {
            let description = self.description.as_ref().unwrap();
            bytes.append(&mut compact_size::encode(byte_helpers::utf8_encode(
                &description,
            )));
        } else {
            bytes.append(&mut vec![0]);
        };
        bytes.append(&mut compact_size::encode(byte_helpers::utf8_encode(&tags)));
        Ok(bytes)
    }

    fn deserialize(bytes: &Vec<u8>) -> Result<Box<Self>, Box<dyn Error>> {
        let decoded = compact_size::decode(&bytes);
        let ptr = byte_helpers::utf8_decode(&decoded[0])?;
        let origin = byte_helpers::utf8_decode(&decoded[1])?;
        let description = byte_helpers::utf8_decode(&decoded[2])?;
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };
        let tags = &byte_helpers::utf8_decode(&decoded[3])?;
        let tags = serde_json::from_str(tags)?;
        Ok(Box::new(Self {
            ptr,
            origin,
            tags,
            description,
        }))
    }
}
