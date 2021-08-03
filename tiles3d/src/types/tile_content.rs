use serde::{Deserialize, Serialize};

use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;
use crate::types::bounding_volume::BoundingVolume;


#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "Tile Content")]
pub struct TileContent {
    #[doc = " An optional bounding volume that tightly encloses just the tile's content. "]
    #[doc = " tile.boundingVolume provides spatial coherence and tile.content.boundingVolume enables "]
    #[doc = " tight view frustum culling. When this is omitted, tile.boundingVolume is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "boundingVolume")]
    pub bounding_volume: Option<BoundingVolume>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,

    /// A uri that points to the tile's content. When the uri is relative, it is relative to the referring tileset JSON file.
    pub uri: String,
}