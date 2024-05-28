use dyntest::{dyntest, DynTester};

dyntest!(test_should_panic, test_ignore);

fn test_should_panic(t: &mut DynTester) {
  t.test("panic", || panic!()).should_panic(true);
  t.test("aaa", || panic!("aaa")).should_panic("aaa");
}

fn test_ignore(t: &mut DynTester) {
  t.test("ignore", || {}).ignore(true);
  t.test("why", || {}).ignore("why not?");
}
