use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use tiles3d::types::tileset::Tileset;

#[test]
pub fn load_test() -> std::io::Result<()> {
    let src = include_str!("tileset.json");

    let value: Tileset = serde_json::from_str(src)?;

    println!("{:#?}", value);

    Ok(())
}