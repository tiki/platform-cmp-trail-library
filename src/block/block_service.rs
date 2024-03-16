/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::{
    super::{utils::S3Client, Owner, Signer},
    BlockModel, ModelTxn,
};
use chrono::{DateTime, Utc};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct BlockService {
    id: Option<String>,
    owner: Owner,
    previous_id: String,
    timestamp: Option<DateTime<Utc>>,
    transactions: Vec<ModelTxn>,
}

#[allow(unused)]
impl BlockService {
    pub fn new(owner: &Owner, previous_id: &str) -> Self {
        BlockService {
            id: None,
            owner: owner.clone(),
            previous_id: previous_id.to_string(),
            timestamp: None,
            transactions: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        timestamp: DateTime<Utc>,
        asset_ref: &str,
        contents: &str,
        user_signature: &str,
        signer: &Signer,
    ) -> Result<&Self, Box<dyn Error>> {
        let txn = ModelTxn::new(
            timestamp,
            asset_ref,
            contents,
            user_signature,
            &self.owner,
            signer,
        );
        self.transactions.push(txn?);
        Ok(self)
    }

    pub async fn write(&mut self, client: &S3Client) -> Result<&Self, Box<dyn Error>> {
        let block =
            BlockModel::write(client, &self.owner, &self.previous_id, &self.transactions).await?;
        self.id = Some(block.id().to_string());
        self.timestamp = Some(block.timestamp());
        self.transactions = block.transactions().clone();
        Ok(self)
    }

    pub async fn read(client: &S3Client, owner: &Owner, id: &str) -> Result<Self, Box<dyn Error>> {
        let block = BlockModel::read(client, owner, id).await?;
        let mut new_block = Self::new(owner, block.previous_id());
        new_block.id = Some(block.id().to_string());
        new_block.timestamp = Some(block.timestamp());
        new_block.transactions = block.transactions().clone();
        Ok(new_block)
    }

    pub fn id(&self) -> &Option<String> {
        &self.id
    }
    pub fn owner(&self) -> &Owner {
        &self.owner
    }
    pub fn previous_id(&self) -> &str {
        &self.previous_id
    }
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }
    pub fn transactions(&self) -> &Vec<ModelTxn> {
        &self.transactions
    }
}
