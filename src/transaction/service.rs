/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

use crate::{byte_helpers, compact_size, Owner, Signer};
use chrono::Utc;
use num_bigint::BigInt;
use super::Model;

pub struct Service{
  address: String,
  signer: Signer,
  transactions: Vec<Model>
}

#[allow(unused)]
impl Service {
  
  pub fn add(
    &self,
    contents: &str,
    asset_ref: &str,
    user_signature: &str,
    version: i32
  ) -> Result<Model, Box<dyn Error>> { 
      let mut bytes = Vec::<u8>::new();
      let version_bigint = &BigInt::from(version);
      bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
          version_bigint,
      )));
      let address = &self.address;
      bytes.append(&mut compact_size::encode(byte_helpers::base64url_decode(address)?)); 
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
      let app_signature = &self.signer.sign(&bytes)?;
      bytes.append(&mut compact_size::encode(app_signature.clone()));
      let id = Self::calculate_id(&bytes);
      let transaction = Model {
          id,
          version,
          address: address.to_string(),
          timestamp,
          asset_ref: asset_ref.to_string(),
          contents: contents.to_string(),
          user_signature: user_signature.to_string(),
          app_signature: byte_helpers::base64_encode(&app_signature),
          bytes,
      };
      &self.transactions.push(transaction);
      return Ok(transaction);
  }

  pub fn calculate_id(bytes: &Vec<u8>) -> String {
    let id = byte_helpers::sha3(&bytes);
    byte_helpers::base64url_encode(&id)
  }

  fn path(owner: &Owner) -> String { 
    match owner.provider() { 
        Some(provider) => {
            match owner.address() {
                Some(address) => format!("providers/{}/{}/metadata.json", provider, address),
                None => format!("providers/{}/metadata.json", provider)
            }
        },
        None => "providers/metadata.json".to_string()
    } 
  }
}
