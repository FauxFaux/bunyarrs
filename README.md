## bunyarrs

`bunyarrs` is a very opinionated, low performance logging library,
modelled on [node bunyan](https://www.npmjs.com/package/bunyan).

```rust
let logger = Bunyarr::with_name("my-module");

let foo = init_foo();
let bar = init_bar();

logger.info(vars! { foo, bar }, "initialisation complete");
```

Will print, to stdout, on one line (whitespace added for the docs):

```json
{
  "time":"2022-09-11T15:19:33.166395524Z",
  "level":30,
  "msg":"initialisation complete",
  "name":"my-module",
  "foo": 5,
  "bar": {
    "baz": 5,
    "badger": "mushroom"
  },
  "hostname":"conqueeftador",
  "pid":1337,
  "v":0
}
```


### ...why?

It's better than `eprintln!` and easier to get going with than `slog`.

This "bunyan" format is supported by a number of log ingest tools. Using it
saves you from attempting to handle objects, multi-line strings and the like
during your log search.

The library is designed to encourage good use of this format, specifically,
there is no support for format strings, or dynamic strings at all, on purpose.

Other tools, such as the [bunyan rust port](https://crates.io/crates/bunyan)
and [pino-pretty](https://www.npmjs.com/package/pino-pretty) exist to turn
these logs back into text, if you want to view them as text. Piping the above
output through `pino-pretty` results in:

```text
[16:19:33.166] INFO (my-module/1337): initialisation complete
    foo: 5
    bar: {
      "baz": 5,
      "badger": "mushroom"
    }
    v: 0
```


### I need support for...

Files, rotation? Nope. Write to stdout, it's well-supported by all orchestration tools.

Threads, performance? Nope. Use the [`slog`](https://crates.io/crates/slog) ecosystem,
and see their justification for the complexities.

Objects, naming? Use the `serde` ecosystem tools.

The existing rust-log ecosystem? Sorry, they use format strings, which is banned.

Custom formatters? Maybe you want to write your own `vars!` macro?


### Should I use this in a rust *library* I want people to use?

No.


### Contributing

`cargo fmt`, `cargo test`, github PRs or issues, please.


### License

MIT OR Apache-2.0
