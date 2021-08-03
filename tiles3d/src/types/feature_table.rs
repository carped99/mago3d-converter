use serde::{Deserialize, Serialize};
use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;

#[doc = " An object defining the reference to a section of the binary body of the features table where "]
#[doc = " the property values are stored if not defined directly in the JSON."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "binaryBodyReference")]
pub struct BinaryBodyReference {
    #[doc = " The offset into the buffer in bytes."]
    #[serde(rename = "byteOffset")]
    pub byte_offset: f64,
    #[doc = " The datatype of components in the property. This is defined only if the semantic allows for "]
    #[doc = " overriding the implicit component type. These cases are specified in each tile format."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
}
#[doc = " An object defining a global boolean property value for all features."]
pub type GlobalPropertyBoolean = bool;

#[doc = " An object defining a global 3-component numeric property values for all features."]
pub type GlobalPropertyCartesian3 = serde_json::Value;

#[doc = " An object defining a global 4-component numeric property values for all features."]
pub type GlobalPropertyCartesian4 = serde_json::Value;

#[doc = " An object defining a global numeric property value for all features."]
pub type GlobalPropertyScalar = serde_json::Value;

pub type NumericArray = Vec<f64>;
#[doc = " A user-defined property which specifies per-feature application-specific metadata in a tile. "]
#[doc = " Values either can be defined directly in the JSON as an array, or can refer to sections in the "]
#[doc = " binary body with a `BinaryBodyReference` object."]
pub type Property = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "Feature Table")]
pub struct FeatureTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,
}

pub enum TileFormat {
    PNTS
}

pub enum BatchTable {
    B3DM,
    I3DM,
    PNTS,
}

pub enum FeatureTableEnum {
    B3DM,
    I3DM,
    PNTS,
}

pub struct B3dmFeatureTable {

}

struct PointCloudHeader {
    magic: [u8; 4],
    version: u32,
    byteLen: u32,
    featureTableJSONLength: u32,
    featureTableBinaryLength: u32,
    batchTableJSONLength: u32,
    batchTableBinaryLength: u32,
}

struct PointCloudFeatureTable {
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_quantized: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgba: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgb: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgb565: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_oct16p: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<BinaryBodyReference>,

    /// A `GlobalPropertyScalar` object defining a numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_length: Option<GlobalPropertyScalar>,

    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_center: Option<GlobalPropertyCartesian3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,
}

struct PointCloudBody {

}