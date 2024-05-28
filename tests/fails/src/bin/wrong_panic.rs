#![cfg(not(test))]

extern crate dyntest;

use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  t.test("oh no", || panic!("aaaaaaaaaa")).should_panic("calmly");
}
