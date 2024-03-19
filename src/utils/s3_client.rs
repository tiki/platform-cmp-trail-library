/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::byte_helpers::base64_encode;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region};
use aws_sdk_s3::Client;
use md5::{Digest, Md5};
use std::{env, error::Error};

#[derive(Clone, Debug)]
pub struct S3Client {
    s3: Client,
    bucket: String,
}

impl S3Client {
    pub async fn new(region: &str, bucket: &str) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_sdk_s3::config::Region::new(String::from(region)))
            .load()
            .await;
        Self {
            s3: Client::new(&config),
            bucket: bucket.to_string(),
        }
    }

    pub async fn from_env() -> Self {
        let region = RegionProviderChain::default_provider()
            .or_else("us-east-2")
            .region()
            .await
            .unwrap();
        let bucket = env::var("TIKI_BUCKET").expect("TIKI_BUCKET is not set");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region)
            .load()
            .await;
        Self {
            s3: Client::new(&config),
            bucket: bucket.to_string(),
        }
    }

    pub async fn read(&self, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let res = self
            .s3
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        let bytes = res.body.collect().await?;
        Ok(bytes.to_vec())
    }

    pub async fn write(&self, key: &str, body: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.s3
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_md5(Self::md5(body))
            .body(aws_sdk_s3::primitives::ByteStream::from(body.clone()))
            .send()
            .await?;
        Ok(())
    }

    fn md5(bytes: &Vec<u8>) -> String {
        let mut hasher = Md5::new();
        hasher.update(bytes);
        let res = hasher.finalize();
        base64_encode(&res[..].to_vec())
    }
}
