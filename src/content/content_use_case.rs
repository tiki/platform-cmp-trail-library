/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::UseCaseType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContentUseCase {
    typ: UseCaseType,
    value: String,
}

impl ContentUseCase {
    pub fn typ(&self) -> &UseCaseType {
        &self.typ
    }
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(string: &str) -> Self {
        let string = string.trim();
        match string {
            "attribution" => Self {
                typ: UseCaseType::Attribution,
                value: string.to_string(),
            },
            "retargeting" => Self {
                typ: UseCaseType::Retargeting,
                value: string.to_string(),
            },
            "personalization" => Self {
                typ: UseCaseType::Personalization,
                value: string.to_string(),
            },
            "ai_training" => Self {
                typ: UseCaseType::AITraining,
                value: string.to_string(),
            },
            "distribution" => Self {
                typ: UseCaseType::Distribution,
                value: string.to_string(),
            },
            "analytics" => Self {
                typ: UseCaseType::Analytics,
                value: string.to_string(),
            },
            "support" => Self {
                typ: UseCaseType::Support,
                value: string.to_string(),
            },
            _ => {
                let value = if string.starts_with("custom:") {
                    string.to_string()
                } else {
                    format!("custom:{}", string)
                };
                Self {
                    typ: UseCaseType::Custom,
                    value,
                }
            }
        }
    }

    pub fn attribution() -> Self {
        Self::new("attribution")
    }
    pub fn retargeting() -> Self {
        Self::new("retargeting")
    }
    pub fn personalization() -> Self {
        Self::new("personalization")
    }
    pub fn ai_training() -> Self {
        Self::new("ai_training")
    }
    pub fn distribution() -> Self {
        Self::new("distribution")
    }
    pub fn analytics() -> Self {
        Self::new("analytics")
    }
    pub fn support() -> Self {
        Self::new("support")
    }
    pub fn custom(string: &str) -> Self {
        Self::new(&format!("customer:{}", string))
    }
}

impl Serialize for ContentUseCase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> Deserialize<'de> for ContentUseCase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(Self::new(&string))
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentUseCase, UseCaseType};

    #[test]
    fn test_deserialize_attribution() {
        let json = "\"attribution\"";
        let use_case: ContentUseCase = serde_json::from_str(json).unwrap();
        assert_eq!(use_case.typ(), &UseCaseType::Attribution);
    }

    #[test]
    fn test_serialize_attribution() {
        let use_case: ContentUseCase = ContentUseCase::new("attribution");
        let json = serde_json::to_string(&use_case).unwrap();
        assert_eq!(json, "\"attribution\"");
    }

    #[test]
    fn test_new_custom_prefix() {
        let use_case: ContentUseCase = ContentUseCase::new("custom:one");
        assert_eq!(use_case.typ(), &UseCaseType::Custom);
        assert_eq!(use_case.value, "custom:one");
    }

    #[test]
    fn test_new_custom_no_prefix() {
        let use_case: ContentUseCase = ContentUseCase::new("one");
        assert_eq!(use_case.typ(), &UseCaseType::Custom);
        assert_eq!(use_case.value, "custom:one");
    }

    #[test]
    fn test_deserialize_custom() {
        let json = "\"custom:one\"";
        let use_case: ContentUseCase = serde_json::from_str(json).unwrap();
        assert_eq!(use_case.typ(), &UseCaseType::Custom);
        assert_eq!(use_case.value, "custom:one");
    }

    #[test]
    fn test_serialize_custom() {
        let use_case: ContentUseCase = ContentUseCase::new("one");
        let json = serde_json::to_string(&use_case).unwrap();
        assert_eq!(json, "\"custom:one\"");
    }
}
