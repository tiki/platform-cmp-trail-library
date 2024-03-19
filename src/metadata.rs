/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod metadata_model;
use metadata_model::MetadataModel;

mod metadata_model_signer;
use metadata_model_signer::MetadataModelSigner;

mod metadata_service;
pub use metadata_service::MetadataService as Metadata;
