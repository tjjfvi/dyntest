use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  for (str, len) in [("a", 1), ("pq", 2), ("xyz", 3)] {
    t.test(str, move || {
      assert_eq!(str.len(), len);
    });
  }
}
