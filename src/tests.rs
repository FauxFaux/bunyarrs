use std::cell::RefCell;

use serde_json::{json, Map, Value};

use crate::{AnyWrite, Bunyarr, Options};

fn test_writer() -> Option<RefCell<Box<dyn AnyWrite>>> {
    Some(RefCell::new(Box::new(Vec::<u8>::new())))
}

fn into_lines(logger: Bunyarr) -> Vec<Map<String, Value>> {
    let vec = logger.into_inner();
    let borrow_checker_demands = AnyWrite::as_any(&**vec.borrow())
        .downcast_ref::<Vec<u8>>()
        .expect("was a valid test writer")
        .split(|&p| p == b'\n')
        // .map(|s| s.trim_ascii())
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_slice(s).unwrap())
        .collect();
    borrow_checker_demands
}

#[test]
fn smoke() {
    let logger = Bunyarr::with_options(Options {
        name: "smoke".to_string(),
        writer: test_writer(),
    });

    logger.log(10, json!({ "hello": 5 }).as_object().unwrap(), "woke");

    let lines = into_lines(logger);

    assert_eq!(1, lines.len());
    assert_eq!(lines[0], serde_json::Map::new());
}
