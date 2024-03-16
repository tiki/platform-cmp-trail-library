/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContentSchemaType {
    Title,
    License,
    Payable,
    Receipt,
}
