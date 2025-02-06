/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressNftInstanceCollection {
    #[serde(rename = "is_unique")]
    pub is_unique: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "holder_address_hash",
        skip_serializing_if = "Option::is_none"
    )]
    pub holder_address_hash: Option<String>,
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "animation_url", skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<String>,
    #[serde(rename = "external_app_url", skip_serializing_if = "Option::is_none")]
    pub external_app_url: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "owner")]
    pub owner: models::AddressParam,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<serde_json::Value>,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl AddressNftInstanceCollection {
    pub fn new(
        is_unique: bool,
        id: String,
        owner: models::AddressParam,
        token_type: String,
        value: String,
    ) -> AddressNftInstanceCollection {
        AddressNftInstanceCollection {
            is_unique,
            id,
            holder_address_hash: None,
            image_url: None,
            animation_url: None,
            external_app_url: None,
            metadata: None,
            owner,
            token: None,
            token_type,
            value,
        }
    }
}
