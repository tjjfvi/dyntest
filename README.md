# `dyntest`

A small Rust library for dynamically creating test cases.

## Usage

```toml
# Cargo.toml
[[test]]
name = "test_name"
harness = false
```
```rust
// tests/test_name.rs

use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  for (str, len) in [("a", 1), ("pq", 2), ("xyz", 3)] {
    t.test(str, move || {
      assert_eq!(str.len(), len);
    });
  }
}
```
```text
running 3 tests
test a ... ok
test pq ... ok
test xyz ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Features

### Test Grouping

Using `DynTester::group`, multiple related tests can be given a shared prefix, akin to a `mod` for static tests:
```rust
use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  panic!("hi");
  t.group("foo", |t| {
    t.group("bar", |t| {
      t.test("baz", || {});
    });
    t.test("qux", || {});
  });
}
```
```text
running 2 tests
test foo::bar::baz ... ok
test foo::qux ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Globbing

When the `glob` feature is enabled (which it is by default), `DynTester` exposes
`glob` and `glob_in` methods, which facilitate generating tests from files in a
directory:
```rust
use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  for (name, path) in t.glob_in("my/test/files/", "**/*.ext") {
    t.test(name, move || {
      // ...
    });
  }
}
```
```text
my/test/files/
  foo.ext
  bar.ext
  baz/
    qux.ext
    something.unrelated
```
```text
running 3 tests
test foo ... ok
test bar ... ok
test baz::qux ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Limitations

Using `dyntest` requires a nightly compiler, as it uses the unstable `test` crate.

In any given test files, the tests must either be all static or all dynamic; if
you use `dyntest!` in a file, any `#[test]` fns will be silently ignored by
rustc (this is inherent to `harness = false`).

Multiple invocations of `dyntest!` in the same test file are not supported;
either separate it into multiple test files, or merge the `dyntest!` invocations
(the macro supports multiple arguments).


