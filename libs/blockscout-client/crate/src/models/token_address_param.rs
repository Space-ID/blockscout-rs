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
pub struct TokenAddressParam {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "implementation_name")]
    pub implementation_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "is_contract")]
    pub is_contract: bool,
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
    #[serde(rename = "private_tags")]
    pub private_tags: Vec<models::AddressTag>,
    #[serde(rename = "public_tags")]
    pub public_tags: Vec<models::AddressTag>,
    #[serde(rename = "watchlist_names")]
    pub watchlist_names: Vec<models::WatchlistName>,
    /// Address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl TokenAddressParam {
    pub fn new(
        hash: String,
        implementation_name: String,
        name: String,
        is_contract: bool,
        is_verified: bool,
        private_tags: Vec<models::AddressTag>,
        public_tags: Vec<models::AddressTag>,
        watchlist_names: Vec<models::WatchlistName>,
    ) -> TokenAddressParam {
        TokenAddressParam {
            hash,
            implementation_name,
            name,
            is_contract,
            is_verified,
            private_tags,
            public_tags,
            watchlist_names,
            address: None,
        }
    }
}
