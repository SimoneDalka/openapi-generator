/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FooEnumArrayTesting : Test of enum array
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FooEnumArrayTesting {
    #[serde(rename = "required_enums")]
    pub required_enums: Vec<RequiredEnums>,
}

impl FooEnumArrayTesting {
    /// Test of enum array
    pub fn new(required_enums: Vec<RequiredEnums>) -> FooEnumArrayTesting {
        FooEnumArrayTesting {
            required_enums,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiredEnums {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
}

impl Default for RequiredEnums {
    fn default() -> RequiredEnums {
        Self::A
    }
}

