/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::compact_size;
use crate::MerkleTree;
use super::{
    super::{byte_helpers, Owner, S3Client, transaction::Model as TxnModel},
    Model
};
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use std::error::Error;

pub struct Service {}

#[allow(unused)]
impl Service {

  pub async fn write(
      client: &S3Client,
      owner: &Owner,
      version: i32,
      previous_id: &str,
      transactions: &Vec<TxnModel>,
  ) -> Result<Model, Box<dyn Error>> {
      let mut transaction_ids = vec![];
      let mut transaction_bytes = vec![];
      for txn in transactions {
          transaction_ids.push(byte_helpers::base64url_decode(txn.id())?);
          transaction_bytes.push(txn.bytes().clone());
      }

      let mut root_tree = MerkleTree::new(&transaction_ids);
      root_tree.build();
      let transaction_root = root_tree
          .root()
          .clone()
          .ok_or("failed to build transaction root")?;

      let mut bytes = Vec::<u8>::new();
      let version_bigint = &BigInt::from(version);
      bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
          version_bigint,
      )));
      let timestamp = Utc::now();
      let timestamp_bigint = &BigInt::from(Utc::now().timestamp());
      bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
          timestamp_bigint,
      )));
      bytes.append(&mut compact_size::encode(byte_helpers::base64url_decode(
          previous_id,
      )?));
      bytes.append(&mut compact_size::encode(transaction_root.clone()));
      let num_transactions = BigInt::from(transactions.len());
      bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
          &num_transactions,
      )));

      transaction_bytes
          .iter()
          .for_each(|txn| bytes.append(&mut compact_size::encode(txn.clone())));

      let id = Self::calculate_id(&bytes);
      let prev_id = previous_id.to_string();
      client.write(&Self::path(owner, &id), &bytes).await?;
      let block = Model{
        id,
        version,
        timestamp,
        previous_id: prev_id,
        transaction_root: byte_helpers::base64_encode(&transaction_root),
        transactions: transactions.clone(),
        bytes
      };

      Ok(block)
  }

  pub async fn read(client: &S3Client, owner: &Owner, id: &str) -> Result<Model, Box<dyn Error>> {
      let bytes = client.read(&Self::path(owner, id)).await?;

      let decoded = compact_size::decode(&bytes);
      let version = byte_helpers::decode_bigint(&decoded[0]);
      let version = version.to_string().parse::<i32>()?;
      let timestamp = byte_helpers::decode_bigint(&decoded[1]);
      let timestamp = DateTime::from_timestamp(timestamp.to_string().parse::<i64>()?, 0)
          .ok_or("Failed to parse timestamp")?;
      let previous_id = byte_helpers::base64url_encode(&decoded[2]);
      let transaction_root = byte_helpers::base64_encode(&decoded[3]);

      let num_transactions = byte_helpers::decode_bigint(&decoded[4]);
      let num_transactions = num_transactions.to_string().parse::<usize>()?;
      let mut transactions: Vec<TxnModel> = vec![];
      for i in 0..num_transactions {
          let transaction = TxnModel::from_bytes(&decoded[5 + i])?;
          transactions.push(transaction);
      }
      Ok(Model {
          id: id.to_string(),
          timestamp,
          version,
          previous_id,
          transaction_root,
          transactions,
          bytes,
      })
  }

  pub fn calculate_id(bytes: &Vec<u8>) -> String {
      let id = byte_helpers::sha3(&bytes);
      byte_helpers::base64url_encode(&id)
  }

  fn path(owner: &Owner, id: &str) -> String {
      match owner.provider() {
          Some(provider) => match owner.address() {
              Some(address) => format!("providers/{}/{}/blocks/{}.block", provider, address, id),
              None => format!("providers/{}/{}.block", provider, id),
          },
          None => format!("providers/{}.block", id),
      }
  }

}