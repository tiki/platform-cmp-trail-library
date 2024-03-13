/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::model::Model;

pub trait Register<T>{
    fn to_transaction(&self) -> Model;
    fn from_transaction(&self) -> T;
}