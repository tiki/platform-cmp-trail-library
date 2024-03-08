/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use chrono::{DateTime, Utc};
use ring::rsa::KeyPair;
use ring::signature;
use super::{ Model, super::{Owner, S3Client, byte_helpers}};

pub struct Service {
    key_pair: KeyPair,
    created: DateTime<Utc>,
    uri: String
}

#[allow(unused)]
impl Service {
    pub async fn create(client: &S3Client, owner: &Owner, key: &str) -> Result<Self, Box<dyn Error>> {
        let path = Self::path(owner);
        let model = Model::write(client, &path, key).await?;
        Ok(Self::from_model(&path, &model)?)
    }

    pub async fn get(client: &S3Client, owner: &Owner) -> Result<Self, Box<dyn Error>> {
        Self::get_from_path(client, &Self::path(owner)).await
    }

    pub async fn get_from_path(client: &S3Client, path: &str) -> Result<Self, Box<dyn Error>> {
        let model = Model::read(client, path).await?;
        Ok(Self::from_model(path, &model)?)
    }

    pub fn sign(&self, message: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut signature = vec![0; self.key_pair.public().modulus_len()];
        match self.key_pair.sign(
            &signature::RSA_PKCS1_SHA256,
            &ring::rand::SystemRandom::new(),
            message.as_slice(),
            &mut signature) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e.to_string())?
        }
    }

    pub fn verify(&self, message: &Vec<u8>, signature: &Vec<u8>) -> bool {
        let pub_key = signature::UnparsedPublicKey::new(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            self.key_pair.public()
        );
        match pub_key.verify(message.as_slice(), signature.as_slice()) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn created(&self) -> DateTime<Utc> { self.created }
    pub fn uri(&self) -> &str { &self.uri }
    pub fn key_pair(&self) -> &KeyPair { &self.key_pair }

    fn from_model(path: &str, model: &Model) -> Result<Self, Box<dyn Error>> {
        let key = byte_helpers::base64_decode(model.key())?;
        match KeyPair::from_der(key.as_slice()) {
            Ok(key_pair) => Ok(Self {
                key_pair, created:
                model.created(),
                uri: path.to_string()
            }),
            Err(e) => Err(e.to_string())?
        }
    }

    fn path(owner: &Owner) -> String { 
        match owner.provider() { 
            Some(provider) => format!("providers/{}/sign.json", provider),
            None => "providers/sign.json".to_string()
        } 
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use tokio_test::assert_ok;
    use crate::api::Owner;
    use super::{Service, super::super::super::utils::S3Client};

    #[tokio::test]
    async fn bully() {
        env::set_var("AWS_PROFILE", "sandbox-mike");
        let client = S3Client::new("us-east-2", "mytiki-sandbox-mike-ocean").await;
        let owner = Owner::new("1234:abcd").unwrap();
        let key = "MIIEowIBAAKCAQEAtRJeTlEFUZvz5YD3z3hNQ9Q386CeIjInfggXffRxpnSzDwaIYvVk5tSe6QkWvM62wbNA1T4pMg4BJsk+PMYPEg4Kuvp3mVUhlxcXx9avRFJ/MBm8K7BO3yYS0tSB81qOwAXaFSkPPuMZwAX781bxPJGycH8NxVIvcJMNshG4ceyDRJ2g66r/y4I7yuBpjb8EiAr6CZfatjtnQanB6zUAk/lN1QR886qfVINTPgrQo3lg3AMgyPvY4oL6+2CYXG4+lbs1ZeeWlYgrF1niDz1WQqm5wnq4OOpW5gQou1S/TUzNwPkViHaLZPR4ELUHAJhTVtnGQj+Yz1YQcCbZclYDpwIDAQABAoIBABA7vtevnNX1sNaFCvJ/OxIXLxL49eRj8bVzXUPOC/hb4we4UrbogCzPqBWFWpP/xlO0Ud4sTf4uUj0bcqh5KzW3q4+az+kfwDyu9dNVNG5gJQX3cK7MBi4sWCito57hIgYmICigfzzomFDmWHG2/Dataz15ro7fsOAbVvF9dDdGUb4ty0wixJ+qgz+ipjzGurH/BDXAC+p96RNjNOUAcbbN0aWvceyR1Qr8ppxzmbzS36G9pxH0w3mBbvf6jVZybv2QEWkrLsBz9w1I/UHI3lqoZjcUsvWcGeHsOTyqyZU6hlH3VaZOA3XfkxvJC20r6UDqfq55paUCuvK0EI+wOp0CgYEA6S00EG75RrcwqYvDqzTeFRpflAUM66XmGZoIrVUUVMZfplCFoOVZnUBvj84mAesSAO9XYgkj3dGIQHRaJurLw3yEoj8JBspcvjjeyxlfQWTtyTl3V9sP1AuqfdXNVqpuOBnUOkC2JMH8sK+TS45JQQmzkVt8PLkbNJjsoCp9LKsCgYEAxsuN1o1KtdumH8IGa0hjeXj3h4io7NF32h0ru2jof74FsLBjme/IsSzncpvehmr+2bMlJDiQT1II2L9iDwR/+bywKQDOZoEiKpC0iTDmgaOv1Ru4PKWcqaxmeGy0xtt2lDy8yGLjmDQJAwitK9PvrTP0W4Gkkl5/WpG1sRbWzPUCgYEAgH3KXOvqLY+xHxzHfEarwpE/7f9CeXB39c+tzGY/x99wweNJX7pCwabKU0JUK2ZwC1fYnUtQGmHmZS44p41nL5gkNovp23YT2TbZfq3CkfSeG+6w1xb+B+lNz/3LI2DTT9Lb4iXauTK0nmCCGHaV42MHMqhpM8UFOtyyOChxxfUCgYBt7Lrmq8RnISkQIIrKIgIXdmNxh4jZQs7CRiUbgVwm3t1ooXDB+0x/ZDQrNLNsopd/q8ba64goxFTt/Y3sffGCF+tVEQQQvFE3NkMOJYMgpnGhJq3Oo0korZMP/hRMbah5OciuCbiOPh/JlK+lL5E1tiflvZ9R7H0BiNbuMJHgTQKBgGnTBLVAU0nrx9V17Hbvce2EztXI/JmvuQpjSlV/zxUMVP4Nh8y56ACTZ7d4QI1NKH7sYi7fHnqzn2Lb8SCoaSSLeG7wVcxASfmIRMF7SFc4ASAto0AKHMpFDYlj77Xqu7ucyxDlqUHuUjYA/mZD0kfBLIVjS51vOonrDpdHd1Dh";

        let signer = Service::create(&client, &owner, key).await;
        assert_ok!(signer);
    }
}