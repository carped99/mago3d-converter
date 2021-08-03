use serde::{Deserialize, Serialize};

#[doc = " An object defining the reference to a section of the binary body of the batch table where the "]
#[doc = " property values are stored if not defined directly in the JSON."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "binaryBodyReference")]
pub struct BinaryBodyReference {
    #[doc = " The offset into the buffer in bytes."]
    #[serde(rename = "byteOffset")]
    pub byte_offset: f64,
    #[doc = " The datatype of components in the property."]
    #[serde(rename = "componentType")]
    pub component_type: String,
    #[doc = " Specifies if the property is a scalar or vector."]
    #[serde(rename = "type")]
    pub type_: String,
}