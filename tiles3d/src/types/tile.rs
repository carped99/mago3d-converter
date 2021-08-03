use serde::{Deserialize, Serialize};

use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;
use crate::types::bounding_volume::BoundingVolume;
use crate::types::tile_content::TileContent;


#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Tile {
    #[doc = " The bounding volume that encloses the tile."]
    #[serde(rename = "boundingVolume")]
    pub bounding_volume: BoundingVolume,

    #[doc = " An array of objects that define child tiles. Each child tile content is fully enclosed by "]
    #[doc = " its parent tile's bounding volume and, generally, has a geometricError less than its parent "]
    #[doc = " tile's geometricError. For leaf tiles, the length of this array is zero, and children may "]
    #[doc = " not be defined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Tile>>,

    #[doc = " Metadata about the tile's content and a link to the content. When this is omitted the tile "]
    #[doc = " is just used for culling. This is required for leaf tiles."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TileContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,

    #[doc = " The error, in meters, introduced if this tile is rendered and its children are not. At "]
    #[doc = " runtime, the geometric error is used to compute screen space error (SSE), i.e., the error "]
    #[doc = " measured in pixels."]
    #[serde(rename = "geometricError")]
    pub geometric_error: f64,

    #[doc = " Specifies if additive or replacement refinement is used when traversing the tileset for "]
    #[doc = " rendering.  This property is required for the root tile of a tileset; it is optional for "]
    #[doc = " all other tiles.  The default is to inherit from the parent tile."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refine: Option<String>,

    #[doc = " A floating-point 4x4 affine transformation matrix, stored in column-major order, that "]
    #[doc = " transforms the tile's content--i.e., its features as well as content.boundingVolume, "]
    #[doc = " boundingVolume, and viewerRequestVolume--from the tile's local coordinate system to the "]
    #[doc = " parent tile's coordinate system, or, in the case of a root tile, from the tile's local "]
    #[doc = " coordinate system to the tileset's coordinate system.  transform does not apply to "]
    #[doc = " geometricError, nor does it apply any volume property when the volume is a region, defined "]
    #[doc = " in EPSG:4979 coordinates."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<f64>>,

    #[doc = " Optional bounding volume that defines the volume the viewer must be inside of before the "]
    #[doc = " tile's content will be requested and before the tile will be refined based on "]
    #[doc = " geometricError."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewerRequestVolume")]
    pub viewer_request_volume: Option<BoundingVolume>,
}