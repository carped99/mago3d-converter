use serde::{Deserialize, Serialize};

use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,
    #[doc = " Application-specific version of this tileset, e.g., for when an existing tileset is "]
    #[doc = " updated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tilesetVersion")]
    pub tileset_version: Option<String>,
    #[doc = " The 3D Tiles version.  The version defines the JSON schema for the tileset JSON and the "]
    #[doc = " base set of tile formats."]
    pub version: String,
}