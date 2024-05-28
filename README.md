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


