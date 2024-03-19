/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod signer_model;
use signer_model::SignerModel;

mod signer_service;
pub use signer_service::SignerService as Signer;
