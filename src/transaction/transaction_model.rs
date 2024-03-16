/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::super::{
    utils::{byte_helpers, compact_size},
    Signer,
};
use num_bigint::BigInt;
use std::error::Error;

fn current_version() -> i32 {
    2
}

#[derive(Debug, Clone)]
pub struct TransactionModel {
    version: i32,
    address: String,
    timestamp: i64,
    asset_ref: String,
    contents: String,
    user_signature: String,
    app_signature: String,
    bytes: Vec<u8>,
}

impl TransactionModel {
    pub fn new(
        address: &str,
        timestamp: i64,
        asset_ref: &str,
        contents: &str,
        user_signature: &str,
        signer: &Signer,
    ) -> Result<Self, Box<dyn Error>> {
        let mut bytes = Vec::<u8>::new();
        let version = current_version();
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
            &BigInt::from(version),
        )));
        bytes.append(&mut compact_size::encode(byte_helpers::base64url_decode(
            &address,
        )?));
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
            &BigInt::from(timestamp),
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
        let app_signature = signer.sign(&bytes)?;
        bytes.append(&mut compact_size::encode(app_signature.clone()));

        Ok(Self {
            version,
            address: address.to_string(),
            timestamp,
            asset_ref: asset_ref.to_string(),
            contents: contents.to_string(),
            user_signature: user_signature.to_string(),
            app_signature: byte_helpers::base64_encode(&app_signature),
            bytes,
        })
    }

    pub fn from(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let decoded = compact_size::decode(&bytes);
        let version = byte_helpers::decode_bigint(&decoded[0]);
        let version = version.to_string().parse::<i32>()?;
        let address = byte_helpers::base64url_encode(&decoded[1]);
        let timestamp = byte_helpers::decode_bigint(&decoded[2]);
        let timestamp = timestamp.to_string().parse::<i64>()?;
        let asset_ref = byte_helpers::utf8_decode(&decoded[3])?;
        let contents = byte_helpers::base64_encode(&decoded[4]);
        let user_signature = byte_helpers::base64_encode(&decoded[5]);
        let app_signature = byte_helpers::base64_encode(&decoded[6]);
        Ok(Self {
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

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn timestamp(&self) -> i64 {
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

    pub fn calculate_id(&self) -> String {
        byte_helpers::base64url_encode(&byte_helpers::sha3(&self.bytes))
    }
}
