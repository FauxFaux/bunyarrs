use std::cell::RefCell;

use serde_json::{json, Map, Value};

use crate::{Bunyarr, Options, WriteImpl};

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

#[test]
fn levels() {
    let make_logger = |min_level_inclusive: u16| {
        Bunyarr::with_options(Options {
            name: "smoke".to_string(),
            writer: test_writer(),
            min_level_inclusive,
        })
    };

    let logger = make_logger(10);
    logger.info((), "hello");
    assert_ne!(into_lines(logger), vec![]);

    let logger = make_logger(30);
    logger.info((), "hello");
    assert_ne!(into_lines(logger), vec![]);

    let logger = make_logger(50);
    logger.info((), "hello");
    assert_eq!(into_lines(logger), vec![]);
}

fn strip_variable(line: &mut Map<String, Value>) {
    assert!(matches!(line.remove("hostname"), Some(Value::String(_))));
    assert!(matches!(line.remove("time"), Some(Value::String(_))));
    assert!(matches!(line.remove("pid"), Some(Value::Number(_))));
}

fn test_writer() -> Option<WriteImpl> {
    Some(WriteImpl::Test(RefCell::new(Vec::new())))
}

fn into_lines(logger: Bunyarr) -> Vec<Map<String, Value>> {
    let vec = logger.into_inner();
    let borrow_checker_demands = match vec {
        WriteImpl::Test(vec) => vec.into_inner(),
        _ => panic!("not test"),
    }
    .split(|&p| p == b'\n')
    // .map(|s| s.trim_ascii())
    .filter(|s| !s.is_empty())
    .map(|s| serde_json::from_slice(s).unwrap())
    .collect();
    borrow_checker_demands
}
