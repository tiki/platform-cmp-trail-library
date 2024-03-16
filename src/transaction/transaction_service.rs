/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::{
    super::{
        utils::{byte_helpers, compact_size},
        Owner, Signer,
    },
    ContentSchema, ContentSerializer, TransactionModel,
};
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct TransactionService {
    id: String,
    schema: ContentSchema,
    model: TransactionModel,
}

#[allow(unused)]
impl TransactionService {
    pub fn new<T: ContentSerializer>(
        owner: &Owner,
        parent: Option<String>,
        schema: &ContentSchema,
        contents: T,
        user_signature: &str,
        signer: &Signer,
    ) -> Result<Self, Box<dyn Error>> {
        let model = TransactionModel::new(
            &Self::calculate_address(owner),
            Utc::now().timestamp(),
            &Self::calculate_asset_ref(parent),
            &Self::serialize_contents(schema, &contents),
            user_signature,
            signer,
        )?;
        Ok(Self {
            id: model.calculate_id(),
            schema: schema.clone(),
            model,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn schema(&self) -> &ContentSchema {
        &self.schema
    }

    pub fn version(&self) -> i32 {
        self.model.version()
    }

    pub fn address(&self) -> &str {
        &self.model.address()
    }

    pub fn timestamp(&self) -> Result<DateTime<Utc>, Box<dyn Error>> {
        DateTime::from_timestamp(self.model.timestamp(), 0).ok_or("Invalid timestamp".into())
    }

    pub fn user_signature(&self) -> &str {
        &self.model.user_signature()
    }

    pub fn app_signature(&self) -> &str {
        &self.model.app_signature()
    }

    pub fn asset_ref(&self) -> &str {
        &self.model.asset_ref()
    }

    pub fn contents<T: ContentSerializer>(&self) -> Result<T, Box<dyn Error>> {
        let contents = byte_helpers::base64_decode(&self.model.contents())?;
        T::deserialize(&contents).map(|res| *res)
    }

    pub fn serialize(&self) -> &Vec<u8> {
        self.model.bytes()
    }

    pub fn deserialize(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let model = TransactionModel::from(bytes)?;
        let contents = byte_helpers::base64_decode(&model.contents())?;
        let contents = compact_size::decode(&contents);
        let schema = ContentSchema::deserialize(&contents[0])?;
        Ok(Self {
            id: model.calculate_id(),
            schema,
            model,
        })
    }

    fn calculate_address(owner: &Owner) -> String {
        match owner.provider() {
            None => byte_helpers::base64url_encode(&byte_helpers::utf8_encode("mytiki.com")),
            Some(provider) => match owner.address() {
                None => byte_helpers::base64url_encode(&byte_helpers::utf8_encode(provider)),
                Some(address) => address.to_string(),
            },
        }
    }

    fn calculate_asset_ref(parent: Option<String>) -> String {
        match parent {
            Some(parent) => format!("txn:://{}", parent),
            None => "".to_string(),
        }
    }

    fn serialize_contents<T: ContentSerializer>(schema: &ContentSchema, contents: &T) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let schema_bigint = &BigInt::from(schema.schema());
        bytes.append(&mut compact_size::encode(byte_helpers::encode_bigint(
            schema_bigint,
        )));
        bytes.append(&mut compact_size::encode(contents.serialize()));
        byte_helpers::base64_encode(&bytes)
    }
}
