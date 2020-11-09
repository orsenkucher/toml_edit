use std::{error::Error, fs};
use toml_edit::Document;

#[test]
fn table_decor() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("tests/fixtures/decor/comment.toml")?;
    let doc = input.parse::<Document>()?;
    let root = doc.as_table();
    let dec = &root.get("data").unwrap().as_table().unwrap().decor;
    assert_eq!(dec.prefix(), "# This is head comment\n");
    assert_eq!(dec.suffix(), " # about data table");
    Ok(())
}

#[test]
fn key_decor() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("tests/fixtures/decor/comment.toml")?;
    let doc = input.parse::<Document>()?;
    let root = doc.as_table();
    let name = root
        .get("data")
        .unwrap()
        .as_table()
        .unwrap()
        .get_kv("name")
        .unwrap();

    let (key, value) = name.decor().unwrap();
    assert_eq!(key.prefix(), "# before name comment\n");
    assert_eq!(value.suffix(), " # config name");
    Ok(())
}
