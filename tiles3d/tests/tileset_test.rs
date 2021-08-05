use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use tiles3d::types::tileset::Tileset;

//use tiles3d::types::tile_content_data::TileContentData;

#[test]
pub fn load_test() -> std::io::Result<()> {
    let src = include_str!("tileset.json");

    let value: Tileset = serde_json::from_str(src)?;

    println!("{:#?}", value);

    Ok(())
}

#[test]
pub fn load_pnts_test() {
    println!("GET PointCloudData");

    let data = include_bytes!("points.pnts");

    //parse_header(res.unwrap());

    // let content_length = res.content_length();
    println!("Content-Length: {:?}", data.len());
}

