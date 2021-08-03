use serde::{Deserialize, Serialize};

use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "boundingVolume")]
pub struct BoundingVolume {
    #[doc = " An array of 12 numbers that define an oriented bounding box.  The first three elements "]
    #[doc = " define the x, y, and z values for the center of the box.  The next three elements (with "]
    #[doc = " indices 3, 4, and 5) define the x axis direction and half-length.  The next three elements "]
    #[doc = " (indices 6, 7, and 8) define the y axis direction and half-length.  The last three elements "]
    #[doc = " (indices 9, 10, and 11) define the z axis direction and half-length."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "box")]
    pub bbox: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,

    #[doc = " An array of six numbers that define a bounding geographic region in EPSG:4979 coordinates "]
    #[doc = " with the order [west, south, east, north, minimum height, maximum height]. Longitudes and "]
    #[doc = " latitudes are in radians, and heights are in meters above (or below) the WGS84 ellipsoid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<f64>>,

    #[doc = " An array of four numbers that define a bounding sphere.  The first three elements define "]
    #[doc = " the x, y, and z values for the center of the sphere.  The last element (with index 3) "]
    #[doc = " defines the radius in meters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sphere: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,
}