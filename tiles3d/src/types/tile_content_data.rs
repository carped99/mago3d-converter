use std::io::{BufRead, BufReader, Read, BufWriter};
//use nom::{IResult,Err,Needed};
use nom::number::streaming::{le_u8,le_u16,le_i16,le_u24,le_i24,le_u32,le_f64};
use std::str::from_utf8;
use serde_json::{from_reader, from_slice};
use serde::{Deserialize, Serialize};

use nom::*;
use crate::types::binary_body_reference::BinaryBodyReference;
use crate::types::extension_schema_json::ExtensionSchemaJson;
use crate::types::extras_schema_json::ExtrasSchemaJson;
use crate::types::feature_table::{GlobalPropertyCartesian3, GlobalPropertyScalar, GlobalPropertyCartesian4};
use std::error::Error;
use nom::lib::std::string::ParseError;
use nom::Err::Failure;
use std::mem::size_of;

pub enum TileContentData {
    B3DM,
    I3DM,
    PNTS,
}

#[derive(Clone,Debug,PartialEq,Eq, Default)]
pub struct PointCloudHeader {
    version: u32,
    byteLen: u32,
    featureTableJSONLength: u32,
    featureTableBinaryLength: u32,
    batchTableJSONLength: u32,
    batchTableBinaryLength: u32,
}

pub struct PointCloudPayload {

}

// impl From<&[u8]> for PointCloudPayload {
//     fn from(input: &[u8]) -> Self {
//         nom::sequence::preceded(nom::bytes::streaming::tag("pnts"), nom::streaming::take(4u32))(input);
//
//         todo!()
//     }
// }


impl PointCloudPayload {

}

pub struct PointCloud {
    header: PointCloudHeader,
    body: PointCloudPayload,
}

/*
named!(pub parse<PointCloud>, do_parse!(
    header: parse_header >>
    (PointCloud {
        header
    })
));
*/

named!(parse_header<PointCloudHeader>, do_parse!(
    magic: read_magic >>
    version: le_u32 >>
    byteLen: le_u32 >>
    featureTableJSONLength: le_u32 >>
    featureTableBinaryLength: le_u32 >>
    batchTableJSONLength: le_u32 >>
    batchTableBinaryLength: le_u32 >>
    (PointCloudHeader {
        version,
        byteLen,
        featureTableJSONLength,
        featureTableBinaryLength,
        batchTableJSONLength,
        batchTableBinaryLength
    })
));

named!(parse_body<PointCloudHeader>, do_parse!(
    magic: read_magic >>
    version: le_u32 >>
    byteLen: le_u32 >>
    featureTableJSONLength: le_u32 >>
    featureTableBinaryLength: le_u32 >>
    batchTableJSONLength: le_u32 >>
    batchTableBinaryLength: le_u32 >>
    (PointCloudHeader {
        version,
        byteLen,
        featureTableJSONLength,
        featureTableBinaryLength,
        batchTableJSONLength,
        batchTableBinaryLength
    })
));

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PointCloudFeatureTable {
    /// A `GlobalPropertyScalar` object defining a numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "POINTS_LENGTH")]
    pub points_length: GlobalPropertyScalar,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "POSITION", skip_serializing_if = "Option::is_none")]
    pub position: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "POSITION_QUANTIZED", skip_serializing_if = "Option::is_none")]
    pub position_quantized: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "RGBA", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "RGB", skip_serializing_if = "Option::is_none")]
    pub rgb: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "RGB565", skip_serializing_if = "Option::is_none")]
    pub rgb565: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "NORMAL", skip_serializing_if = "Option::is_none")]
    pub normal: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "NORMAL_OCT16P", skip_serializing_if = "Option::is_none")]
    pub normal_oct16p: Option<BinaryBodyReference>,

    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "BATCH_ID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<BinaryBodyReference>,

    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "RTC_CENTER", skip_serializing_if = "Option::is_none")]
    pub rtc_center: Option<GlobalPropertyCartesian3>,

    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "QUANTIZED_VOLUME_OFFSET", skip_serializing_if = "Option::is_none")]
    pub quantized_volume_offset: Option<GlobalPropertyCartesian3>,

    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "QUANTIZED_VOLUME_SCALE", skip_serializing_if = "Option::is_none")]
    pub quantized_volume_scale: Option<GlobalPropertyCartesian3>,

    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "CONSTANT_RGBA", skip_serializing_if = "Option::is_none")]
    pub constant_rgba: Option<GlobalPropertyCartesian4>,

    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points.
    /// See the corresponding property semantic in [Semantics](/specification/TileFormats/PointCloud/README.md#semantics).
    #[serde(rename = "BATCH_LENGTH	", skip_serializing_if = "Option::is_none")]
    pub batch_length: Option<GlobalPropertyScalar>,

    #[serde(rename = "RGB", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ExtensionSchemaJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<ExtrasSchemaJson>,
}

fn read_position(featureTable: &PointCloudFeatureTable, input: &[u8]) {
    let points= featureTable.points_length.as_u64().unwrap() as usize;
    if let Some(x) = &featureTable.position {
        //x.byte_offset +
        let i = size_of::<f32>() * points;
        let result: IResult<&[u8], &[u8]> = bytes::streaming::take(i)(input);
        //println!("{}", result);
    } else if let Some(x) = &featureTable.position_quantized {

    } else {

    }
}



pub fn read_featureTable<'a>(header: &'a PointCloudHeader, input: &'a [u8]) -> IResult<&'a [u8],&'a [u8]> {
    println!("featureTableJSONLength: {:?}", header.featureTableJSONLength);

    let payload = match bytes::streaming::take(header.featureTableJSONLength)(input) {
        Ok(result) => result,
        Err(e) => return Err(e)
    };

    let featureTableJson: PointCloudFeatureTable = match from_slice(payload.1) {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(nom::Err::Error(error::Error::new(input, nom::error::ErrorKind::ParseTo)))
        }
    };

    println!("featureTableJson: {:?}", featureTableJson);

    read_position(&featureTableJson, input);

    println!("read batchTableJSONLength: {}", header.batchTableJSONLength);
    let payload = match bytes::streaming::take(header.batchTableJSONLength)(payload.0) {
        Ok((remaining, payload)) => Ok((remaining, payload)),
        Err(e) => return Err(e)
    };

    //println!("body batchTableJSONLength: {:?}", payload.1);

    payload
}

fn read_magic(i: &[u8]) -> nom::IResult<&[u8], &str> {
    match nom::bytes::streaming::tag("pnts")(i) {
        Ok((remains, _)) => Ok((remains, "pnts")),
        Err(e) => Err(e)
    }
}

fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::Error;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        println!("GET PointCloudData");

        let data = include_bytes!("points.pnts");

        //let data = &data[0 .. 400];

        //PointCloudPayload::from(data);

        match parse_header(data) {
            Ok((remaining, header)) => {
                //println!("{:?}", header);
                println!("read featureTable");
                match read_featureTable(&header, remaining) {
                    Ok(a) => {
                        //serde_json::from_slice(a.1);
                        println!("{:?}", a.1.len());
                    },
                    Err(e) => {
                        panic!("There was a problem opening the file: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("There was a problem opening the file: {:?}", e);
            }
        }

        //println!("{:?}", parse_header(data));

        // let xs: [u8; 5] = [1, 2, 3, 4, 5];
        // println!("{:?}", magic_parser(&xs));
        // println!("{:?}", hello_parser("hello"));
        // println!("{:?}", hello_parser("hello world"));
        // println!("{:?}", hello_parser("goodbye hello again"));

        /*
        let mut res = reqwest::blocking::get("https://raw.githubusercontent.com/CesiumGS/3d-tiles-samples/e13380168ea1bb077382d9e61d94823bba7411e3/tilesets/TilesetWithExpiration/points.pnts")?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:?}", res.headers());

        let bytes = res.bytes().unwrap().to_vec();

        let x = bytes.as_slice();

        let result = parse_header(x);
        println!("\n\nDone. {:?}", result);

        // copy the response body directly to stdout
        //res.copy_to(&mut std::io::stdout())?;

        println!("\n\nDone. {}", bytes.len());
         */
        Ok(())
    }
}