// This uses the standard test harness to test the `dyntest` harness; this isn't
// a usage example.

use std::process::Command;

#[test]
fn test_examples() {
  test_example("basic", &[], [3, 0, 0, 0, 0]);
  test_example("config", &[], [2, 0, 2, 0, 0]);
  test_example("glob", &[], [4, 0, 0, 0, 0]);
  test_example("groups", &[], [2, 0, 0, 0, 0]);

  test_example("config", &["--include-ignored"], [4, 0, 0, 0, 0]);
  test_example("groups", &["bar"], [1, 0, 0, 0, 1]);
}

#[test]
fn test_fails() {
  test_fail("basic_panic", &[], [0, 1, 0, 0, 0]);
  test_fail("wrong_panic", &[], [0, 1, 0, 0, 0]);
  test_fail("ignored_failure", &[], [0, 0, 1, 0, 0]);
  test_fail("ignored_failure", &["--include-ignored"], [0, 1, 0, 0, 0]);
  test_fail("basic_panic", &["nothing"], [0, 0, 0, 0, 1]);
}

fn test_example(name: &str, harness_args: &[&str], stats: [usize; 5]) {
  test(&["--example", name], harness_args, stats)
}

fn test_fail(name: &str, harness_args: &[&str], stats: [usize; 5]) {
  test(&["-p", "fails", "--bin", name], harness_args, stats)
}

fn test(
  cargo_args: &[&str],
  harness_args: &[&str],
  [passed, failed, ignored, measured, filtered_out]: [usize; 5],
) {
  println!("-------------------");
  println!("testing: {cargo_args:?} {harness_args:?}");
  let output = Command::new("cargo")
    .arg("run")
    .args(cargo_args)
    .arg("--")
    .args(harness_args)
    .output()
    .unwrap();
  let output = std::str::from_utf8(&output.stdout).unwrap();
  let expected = format!(
    "{passed} passed; {failed} failed; {ignored} ignored; {measured} measured; {filtered_out} filtered out"
  );
  println!("output: {output}");
  println!("expected: {expected}");
  assert!(output.contains(&expected));
}
