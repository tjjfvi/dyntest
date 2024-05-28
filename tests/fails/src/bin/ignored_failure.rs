#![cfg(not(test))]

extern crate dyntest;

use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  t.test("meh", || panic!("surprise!")).ignore("it's probably fine");
}
