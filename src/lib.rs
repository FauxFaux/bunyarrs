use std::any::Any;
use std::cell::RefCell;
use std::io::Write;
use std::ops::DerefMut;

use serde_json::{json, Value};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// #[cfg(test)]
// mod tests;

pub struct Bunyarr {
    writer: RefCell<Box<dyn AnyWrite>>,
    name: String,
}

pub(crate) struct Options {
    pub(crate) name: String,
    pub(crate) writer: Option<RefCell<Box<dyn AnyWrite>>>,
}

lazy_static::lazy_static! {
    static ref PROC_INFO: ProcInfo = ProcInfo::new();
}

impl Bunyarr {
    pub fn with_name(name: impl ToString) -> Bunyarr {
        Self::with_options(Options {
            name: name.to_string(),
            writer: None,
        })
    }

    pub(crate) fn with_options(options: Options) -> Bunyarr {
        Bunyarr {
            writer: options.writer.unwrap_or_else(default_writer),
            name: options.name,
        }
    }

    pub(crate) fn log<'a>(
        &self,
        level: u16,
        extras: impl IntoIterator<Item = (impl ToString, &'a Value)>,
        msg: impl AsRef<str>,
    ) {
        // https://github.com/trentm/node-bunyan#core-fields
        let mut obj = serde_json::Map::<String, Value>::with_capacity(12);
        obj.insert(
            "time".to_string(),
            Value::String(
                OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .expect("built-in time and formatter"),
            ),
        );
        for (key, value) in extras {
            obj.insert(key.to_string(), value.clone());
        }
        obj.insert("v".to_string(), json!(0));
        obj.insert("msg".to_string(), json!(msg.as_ref()));
        obj.insert("level".to_string(), json!(level));
        obj.insert("hostname".to_string(), json!(PROC_INFO.hostname));
        obj.insert("pid".to_string(), json!(PROC_INFO.pid));
        obj.insert("name".to_string(), json!(self.name));
        let mut writer = RefCell::borrow_mut(&self.writer);
        let _ = serde_json::to_writer(writer.deref_mut(), &obj);
        let _ = writer.write_all(b"\n");
    }

    #[cfg(test)]
    pub(crate) fn into_inner(self) -> RefCell<Box<dyn AnyWrite>> {
        self.writer
    }
}

fn default_writer() -> RefCell<Box<dyn AnyWrite>> {
    RefCell::new(Box::new(std::io::stdout()))
}

trait AnyWrite: Write + Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Write + Any> AnyWrite for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ProcInfo {
    hostname: String,
    pid: u32,
}

impl ProcInfo {
    fn new() -> ProcInfo {
        ProcInfo {
            // non-utf8 hostname: ignored
            hostname: gethostname::gethostname().into_string().unwrap_or_default(),
            pid: std::process::id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bunyarr, Options};
    use serde_json::{json, Value};
    use std::any::Any;
    use std::cell::RefCell;

    #[test]
    fn smoke() {
        let logger = Bunyarr::with_options(Options {
            name: "smoke".to_string(),
            writer: Some(RefCell::new(Box::new(Vec::<u8>::new()))),
        });

        logger.log(10, json!({ "hello": 5 }).as_object().unwrap(), "woke");

        let vec = logger.into_inner();
        let obj: serde_json::Map<String, Value> =
            serde_json::from_slice(vec.borrow().as_any().downcast_ref::<Vec<u8>>().unwrap())
                .unwrap();

        assert_eq!(obj, serde_json::Map::new());
    }
}
