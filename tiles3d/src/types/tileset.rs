use serde::{Deserialize, Serialize};

use crate::types::asset::Asset;
use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;
use crate::types::properties::Properties;
use crate::types::tile::Tile;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Tileset {
    pub asset: Asset,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,

    #[doc = " Names of 3D Tiles extensions required to properly load this tileset."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extensionsRequired")]
    pub extensions_required: Option<Vec<String>>,

    #[doc = " Names of 3D Tiles extensions used somewhere in this tileset."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extensionsUsed")]
    pub extensions_used: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,

    #[doc = " The error, in meters, introduced if this tileset is not rendered. At runtime, the geometric "]
    #[doc = " error is used to compute screen space error (SSE), i.e., the error measured in pixels."]
    #[serde(rename = "geometricError")]
    pub geometric_error: f64,

    #[doc = " A dictionary object of metadata about per-feature properties."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::BTreeMap<String, Properties>>,

    #[doc = " The root tile."]
    pub root: Tile,
}