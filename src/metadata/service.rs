/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use super::{model::current_version, Model, ModelSigner, super::{Signer, Owner, S3Client}};

pub struct Service {
    client: S3Client,
    owner:Owner,
    metadata: Model
}

#[allow(unused)]
impl Service {

  pub async fn initialize(
      &mut self,
      client: &S3Client,
      parent: Option<String>,
      owner: &Owner
  ) -> Result<Model, Box<dyn Error>> {
      let last_block = parent.unwrap_or("AA".to_string());
      let signer: Signer = Signer::get(client, owner).await?;
      let signers: Vec<ModelSigner> = vec![ModelSigner::new(signer.uri(), signer.created())];
      let model = self.write(owner, &last_block, vec![], signers).await?;
      self.metadata = model.clone();
      Ok(model)
  }


  pub async fn write(
    &self,
    owner: &Owner,
    last_block: &str,
    blocks: Vec<String>,
    signers: Vec<ModelSigner>) -> Result<Model, Box<dyn Error>> {
    let now = Utc::now();
    let model = Model {
        version: current_version(),
        owner: owner.clone(),
        last_block: last_block.to_string(),
        blocks,
        signers,
        modified: now, created: now
    };
    let path = Self::path(owner);
    let body = serde_json::to_string(&model)?.as_bytes().to_vec();
    self.client.write(&path, &body).await?;
    Ok(model)
  }

  

  pub async fn add_block(
      &mut self,
      client: &S3Client,
      owner: &Owner,
      block: &str
  ) -> Result<&Self, Box<dyn Error>> {
      let mut blocks = self.blocks().clone();
      blocks.push(block.to_string());
      self.metadata.blocks = blocks;
      self.metadata.last_block = block.to_string();
      self.metadata.modified = Utc::now();
      let signers = self.signers().iter()
          .map(|s| ModelSigner::new(s.uri(), s.created()))
          .collect();
      self.write(owner, block, self.metadata.blocks.clone(), signers).await?;
      Ok(self)
  }

  pub async fn read(client: &S3Client, owner: &Owner) -> Result<Model, Box<dyn Error>> {
      let path = Self::path(owner);
      let body = client.read(&path).await?;
      let res:Model = serde_json::from_str(&String::from_utf8(body)?)?;
      Ok(res)
  }

  pub async fn get(&self, client: &S3Client, owner: &Owner) -> Result<Model, Box<dyn Error>> {
    let model = Service::read(client, owner).await?;
    let signers = Self::get_signers(client, &model).await?;
    Ok(model)
}

  async fn get_signers(client: &S3Client, model: &Model) -> Result<Vec<Signer>, Box<dyn Error>> {
        let mut signers: Vec<Signer> = Vec::new();
        for s in model.signers() {
            let signer = Signer::get_from_path(client, s.uri()).await?;
            signers.push(signer);
        }
        Ok(signers)
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

  pub fn version(&self) -> i32 { self.metadata.version() }
  pub fn last_block(&self) -> &str { self.metadata.last_block() }
  pub fn owner(&self) -> &Owner { &self.owner }
  pub fn modified(&self) -> DateTime<Utc> { self.metadata.modified() }
  pub fn created(&self) -> DateTime<Utc> { self.metadata.created() }
  pub fn blocks(&self) -> &Vec<String> { self.metadata.blocks() }
  pub fn signers(&self) -> &Vec<ModelSigner> { self.metadata.signers() }
}

