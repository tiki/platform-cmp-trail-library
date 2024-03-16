/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContentUseCaseType {
    Attribution,
    Retargeting,
    Personalization,
    AITraining,
    Distribution,
    Analytics,
    Support,
    Custom,
}
