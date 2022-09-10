use std::any::Any;
use std::cell::RefCell;
use std::io::Write;
use std::ops::DerefMut;

use serde_json::{json, Value};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::extras::Extras;

mod extras;

#[cfg(test)]
mod tests;

pub struct Bunyarr {
    writer: RefCell<Box<dyn AnyWrite>>,
    name: String,
    min_level_inclusive: u16,
}

pub(crate) struct Options {
    pub(crate) name: String,
    pub(crate) writer: Option<RefCell<Box<dyn AnyWrite>>>,
    pub(crate) min_level_inclusive: u16,
}

lazy_static::lazy_static! {
    static ref PROC_INFO: ProcInfo = ProcInfo::new();
}

impl Bunyarr {
    pub fn with_name(name: impl ToString) -> Bunyarr {
        Self::with_options(Options {
            name: name.to_string(),
            writer: None,
            min_level_inclusive: PROC_INFO.min_level_inclusive,
        })
    }

    pub(crate) fn with_options(options: Options) -> Bunyarr {
        Bunyarr {
            writer: options.writer.unwrap_or_else(default_writer),
            name: options.name,
            min_level_inclusive: options.min_level_inclusive,
        }
    }

    #[inline]
    pub fn debug(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive < 20 {
            return;
        }
        self.log(20, extras, event_type)
    }

    #[inline]
    pub fn info(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive < 30 {
            return;
        }
        self.log(30, extras, event_type)
    }

    #[inline]
    pub fn warn(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive < 40 {
            return;
        }
        self.log(40, extras, event_type)
    }

    #[inline]
    pub fn error(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive < 50 {
            return;
        }
        self.log(50, extras, event_type)
    }

    #[inline]
    pub fn fatal(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive < 60 {
            return;
        }
        self.log(60, extras, event_type)
    }

    pub(crate) fn log(&self, level: u16, extras: impl Extras, event_type: &'static str) {
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
        for (key, value) in extras.to_extras() {
            obj.insert(key, value);
        }
        obj.insert("v".to_string(), json!(0));
        obj.insert("msg".to_string(), json!(event_type));
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
    min_level_inclusive: u16,
}

impl ProcInfo {
    fn new() -> ProcInfo {
        ProcInfo {
            // non-utf8 hostname: ignored
            hostname: gethostname::gethostname().into_string().unwrap_or_default(),
            pid: std::process::id(),
            min_level_inclusive: std::env::var("LOG_LEVEL")
                .map(|s| match s.to_ascii_lowercase().as_ref() {
                    "debug" => 20,
                    "info" => 30,
                    "warn" => 40,
                    "error" => 50,
                    "fatal" => 60,
                    _ => 30,
                })
                .unwrap_or(30),
        }
    }
}
