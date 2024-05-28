use std::fs;

use dyntest::{dyntest, DynTester};

dyntest!(test);

fn test(t: &mut DynTester) {
  t.group("uses_dyntest", |t| {
    for (name, path) in t.glob_in("examples", "**.rs") {
      t.test(name, move || {
        // All the usage examples should use `dyntest!`.
        assert!(fs::read_to_string(path).unwrap().contains("dyntest!"));
      });
    }
  });
}
