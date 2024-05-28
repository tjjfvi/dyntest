use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  t.group("foo", |t| {
    t.group("bar", |t| {
      t.test("baz", || {});
    });
    t.test("qux", || {});
  });
}
