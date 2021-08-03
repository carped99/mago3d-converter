use serde_json::{Result, Value};

use tiles3d::types::bounding_volume::BoundingVolume;

#[test]
fn it_adds_two() -> Result<()> {
    let data = r#"
{
  "region": [
    -1.3197004795898053,
    0.6988582109,
    -1.3196595204101946,
    0.6988897891,
    0,
    20
  ]
}
"#;
    let v: BoundingVolume = serde_json::from_str(data)?;
    println!("Please call {:#?}", v);
    //println!("Parsed Value {:#?}", value);
    //assert_eq!(4, adder::add_two(2));

    Ok(())
}