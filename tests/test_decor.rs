use std::{error::Error, fs};
use toml_edit::{Decor, Document};

struct Test {
    doc: Document,
}

fn given(input: &str) -> Test {
    let doc = input.parse::<Document>();
    assert!(doc.is_ok());
    Test { doc: doc.unwrap() }
}

impl Test {
    fn root_decor(&self) -> &Decor {
        &self.doc.as_table().decor
    }
}

#[test]
fn key_prefix() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("tests/fixtures/decor/comment.toml")?;
    println!("input:\n{}", input);
    let test = given(&input);
    let dec = test.root_decor();
    println!("{:?}", dec);
    assert_eq!(dec.prefix(), "# This is head comment");
    Ok(())
}
