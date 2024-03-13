/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use chrono::{DateTime, Utc};

use crate::{byte_helpers, compact_size};

use std::error::Error;

use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Model {
    pub id: String,
    pub version: i32,
    pub address: String,
    pub timestamp: DateTime<Utc>,
    pub asset_ref: String,
    pub contents: String,
    pub user_signature: String,
    pub app_signature: String,
    pub bytes: Vec<u8>,
}

impl Model{

  pub fn from_bytes(bytes: &Vec<u8>) -> Result<Model, Box<dyn Error>> {
    let decoded = compact_size::decode(bytes);
    let version = byte_helpers::decode_bigint(&decoded[0]);
    let version = version.to_string().parse::<i32>()?;
    let address = byte_helpers::base64url_encode(&decoded[1]);
    let timestamp = byte_helpers::decode_bigint(&decoded[2]);
    let timestamp = DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
        .ok_or("Failed to parse timestamp")?;
    let asset_ref = byte_helpers::utf8_decode(&decoded[3])?;
    let contents = byte_helpers::base64_encode(&decoded[4]);
    let user_signature = byte_helpers::base64_encode(&decoded[5]);
    let app_signature = byte_helpers::base64_encode(&decoded[6]);
    let id = Transaction::calculate_id(bytes);
    Ok(Self {
        id,
        version,
        address,
        timestamp,
        asset_ref,
        contents,
        user_signature,
        app_signature,
        bytes: bytes.clone(),
    })
}

  pub fn id(&self) -> &str {
    &self.id
  }
  pub fn version(&self) -> i32 {
      self.version
  }
  pub fn address(&self) -> &str {
      &self.address
  }
  pub fn timestamp(&self) -> DateTime<Utc> {
      self.timestamp
  }
  pub fn asset_ref(&self) -> &str {
      &self.asset_ref
  }
  pub fn contents(&self) -> &str {
      &self.contents
  }
  pub fn user_signature(&self) -> &str {
      &self.user_signature
  }
  pub fn app_signature(&self) -> &str {
      &self.app_signature
  }
  pub fn bytes(&self) -> &Vec<u8> {
      &self.bytes
  }
}