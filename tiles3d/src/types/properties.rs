use serde::{Deserialize, Serialize};

use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Properties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,
    #[doc = " The maximum value of this property of all the features in the tileset."]
    pub maximum: f64,
    #[doc = " The minimum value of this property of all the features in the tileset."]
    pub minimum: f64,
}