use std::io;
use std::io::Write;

use serde_json::{json, Value};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::extras::Extras;

mod extras;

#[cfg(test)]
mod tests;

pub struct Bunyarr {
    writer: WriteImpl,
    name: String,
    min_level_inclusive: u16,
}

enum WriteImpl {
    StdOut,
    #[cfg(test)]
    Test(std::cell::RefCell<Vec<u8>>),
}

impl WriteImpl {
    fn work(&self, f: impl FnOnce(&mut dyn Write) -> io::Result<()>) -> io::Result<()> {
        match self {
            WriteImpl::StdOut => {
                let mut w = io::stdout().lock();
                f(&mut w)
            }
            #[cfg(test)]
            WriteImpl::Test(vec) => f(&mut *vec.borrow_mut()),
        }
    }
}

pub(crate) struct Options {
    pub(crate) name: String,
    pub(crate) writer: Option<WriteImpl>,
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
        if self.min_level_inclusive > 20 {
            return;
        }
        self.log(20, extras, event_type)
    }

    #[inline]
    pub fn info(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive > 30 {
            return;
        }
        self.log(30, extras, event_type)
    }

    #[inline]
    pub fn warn(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive > 40 {
            return;
        }
        self.log(40, extras, event_type)
    }

    #[inline]
    pub fn error(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive > 50 {
            return;
        }
        self.log(50, extras, event_type)
    }

    #[inline]
    pub fn fatal(&self, extras: impl Extras, event_type: &'static str) {
        if self.min_level_inclusive > 60 {
            return;
        }
        self.log(60, extras, event_type)
    }

    pub(crate) fn log(&self, level: u16, extras: impl Extras, event_type: &'static str) {
        // https://github.com/trentm/node-bunyan#core-fields
        // allowing overwriting of things disallowed by bunyan, not particularly concerned, prefer the order
        let mut obj = serde_json::Map::<String, Value>::with_capacity(12);
        obj.insert(
            "time".to_string(),
            Value::String(
                OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .expect("built-in time and formatter"),
            ),
        );
        obj.insert("level".to_string(), json!(level));
        obj.insert("msg".to_string(), json!(event_type));
        obj.insert("name".to_string(), json!(self.name));
        for (key, value) in extras.to_extras() {
            obj.insert(key, value);
        }
        obj.insert("hostname".to_string(), json!(PROC_INFO.hostname));
        obj.insert("pid".to_string(), json!(PROC_INFO.pid));
        obj.insert("v".to_string(), json!(0));
        let _ = self.writer.work(|mut w| {
            serde_json::to_writer(&mut w, &obj)?;
            w.write_all(b"\n")
        });
    }

    #[cfg(test)]
    pub(crate) fn into_inner(self) -> WriteImpl {
        self.writer
    }
}

fn default_writer() -> WriteImpl {
    WriteImpl::StdOut
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
