use std::any::Any;
use std::cell::RefCell;
use std::io::Write as _;

use serde_json::{json, Map, Value};

use crate::{AnyWrite, Bunyarr, Options};

fn test_writer() -> Option<RefCell<Box<dyn AnyWrite>>> {
    Some(RefCell::new(Box::new(Vec::<u8>::new())))
}

fn into_lines(logger: Bunyarr) -> Vec<Map<String, Value>> {
    let vec = logger.into_inner();
    let mut result = Vec::new();
    for line in (*vec.borrow())
        .as_any()
        .downcast_ref::<Vec<u8>>()
        .expect("was a valid test writer")
        .split(|&p| p == b'\n')
    {
        result.push(serde_json::from_slice(line).unwrap());
    }

    result
}

#[test]
fn smoke() {
    let logger = Bunyarr::with_options(Options {
        name: "smoke".to_string(),
        writer: test_writer(),
    });

    logger.log(10, json!({ "hello": 5 }).as_object().unwrap(), "woke");

    let lines = into_lines(logger);

    assert_eq!(lines[0], serde_json::Map::new());
}
