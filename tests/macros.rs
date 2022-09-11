use bunyarrs::{vars, vars_dbg};
use serde_json::json;

#[derive(Default, serde::Serialize)]
struct Example {
    a: u64,
    b: Option<String>,
}

#[test]
fn varies() {
    let an_int = 5;
    let a_string = "hello";
    let ex = Example::default();
    assert_eq!(vars! { an_int }, json!({ "an_int": 5 }));
    assert_eq!(
        vars! { an_int, a_string },
        json!({ "an_int": 5, "a_string": "hello" })
    );

    assert_eq!(vars! { an_int, }, json!({ "an_int": 5 }));
    assert_eq!(
        vars! { an_int, a_string, },
        json!({ "an_int": 5, "a_string": "hello" })
    );

    assert_eq!(
        vars! { ex },
        json!({ "ex": {
            "a": 0,
            "b": null,
        }})
    );
}

#[test]
fn varies_dbg() {
    let an_int = 5;
    let a_string = "hello";
    assert_eq!(vars_dbg! { an_int }, json!({ "an_int": "5" }));
    assert_eq!(
        vars_dbg! { an_int, a_string },
        json!({ "an_int": "5", "a_string": "\"hello\"" })
    );

    assert_eq!(vars_dbg! { an_int, }, json!({ "an_int": "5" }));
    assert_eq!(
        vars_dbg! { an_int, a_string, },
        json!({ "an_int": "5", "a_string": "\"hello\"" })
    );
}
