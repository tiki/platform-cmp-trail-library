/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Debug, PartialEq, Eq)]
pub enum WriterGroupType {
    Initialize,
    Transaction,
}
