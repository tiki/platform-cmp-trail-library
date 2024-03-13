/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::{byte_helpers, compact_size, Block, Metadata, Signer};
use chrono::Utc;
use num_bigint::BigInt;
use std::error::Error;
use super::Model;

pub struct Service{
  address: String,
  signer: Signer,
  transactions: Vec<Model>
}

#[allow(unused)]
impl Service {
  
  pub fn add(
    contents: &str,
    asset_ref: &str,
    user_signature: &str,
  ) -> Result<Model, Box<dyn Error>> {
      let mut bytes = Vec::<u8>::new();
      let version = Metadata::current_version();
      let version_bigint = &BigInt::from(Metadata::current_version());
      bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
          version_bigint,
      )));
      let address = Self.address;
      bytes.append(&mut compact_size::encode(byte_helpers::base64url_decode(
          Self.address,
      )?));
      let timestamp = Utc::now();
      let timestamp_bigint = &BigInt::from(timestamp.timestamp());
      bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
          timestamp_bigint,
      )));
      bytes.append(&mut compact_size::encode(byte_helpers::utf8_encode(
          asset_ref,
      )));
      bytes.append(&mut compact_size::encode(byte_helpers::base64_decode(
          contents,
      )?));
      bytes.append(&mut compact_size::encode(byte_helpers::base64_decode(
          user_signature,
      )?));
      let app_signature = Self.signer.sign(&bytes)?;
      bytes.append(&mut compact_size::encode(app_signature.clone()));
      let id = Self::calculate_id(&bytes);
      let transaction = Model::new {
          id,
          version,
          address,
          timestamp,
          asset_ref: asset_ref.to_string(),
          contents: contents.to_string(),
          user_signature: user_signature.to_string(),
          app_signature: byte_helpers::base64_encode(&app_signature),
          bytes,
      };
  }

  fn calculate_id(bytes: &Vec<u8>) -> String {
    let id = byte_helpers::sha3(&bytes);
    byte_helpers::base64url_encode(&id)
  }

}
