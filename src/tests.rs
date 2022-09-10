use std::cell::RefCell;

use serde_json::{json, Map, Value};

use crate::{AnyWrite, Bunyarr, Options};

#[test]
fn smoke() {
    let logger = Bunyarr::with_options(Options {
        name: "smoke".to_string(),
        writer: test_writer(),
        min_level_inclusive: 10,
    });

    logger.log(10, json!({ "hello": 5 }), "woke");

    let mut lines = into_lines(logger).into_iter();
    let mut line = lines.next().expect("one line");

    strip_variable(&mut line);
    assert_eq!(
        &line,
        json!({
            "hello": 5,
            "msg": "woke",
            "name": "smoke",
            "v": 0,
            "level": 10,
        })
        .as_object()
        .unwrap()
    );
    assert_eq!(lines.next(), None);
}

fn strip_variable(line: &mut Map<String, Value>) {
    assert!(matches!(line.remove("hostname"), Some(Value::String(_))));
    assert!(matches!(line.remove("time"), Some(Value::String(_))));
    assert!(matches!(line.remove("pid"), Some(Value::Number(_))));
}

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
