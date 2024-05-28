#![doc = include_str!("../README.md")]
#![feature(test)]

extern crate test;

use std::{
  borrow::Cow,
  env,
  path::{Path, PathBuf},
  process::Termination,
};
use test::{test_main, TestDesc, TestDescAndFn, TestFn, TestName, TestType};

/// Enables the `dyntest` test runner, given a list of test-generating
/// functions.
#[macro_export]
macro_rules! dyntest {
  ($f:expr $(,)?) => {
    fn main() {
      $crate::_dyntest(env!("CARGO_MANIFEST_DIR"), file!(), line!() as usize, column!() as usize, $f)
    }
  };
  ($($f:ident),+ $(,)?) => {
    $crate::dyntest!(|tester| {
      $(tester.group(stringify!($f), $f);)*
    });
  };
}

#[doc(hidden)]
pub fn _dyntest(
  manifest: &'static str,
  file: &'static str,
  line: usize,
  col: usize,
  f: impl FnOnce(&mut DynTester),
) {
  let mut tester = DynTester { manifest, file, line, col, tests: vec![], group: String::new() };
  f(&mut tester);
  let args = env::args().collect::<Vec<_>>();
  let tests = tester.tests.into_iter().map(|x| x.0).collect();
  test_main(&args, tests, None)
}

/// A dynamic test harness.
pub struct DynTester {
  manifest: &'static str,
  file: &'static str,
  line: usize,
  col: usize,
  tests: Vec<DynTest>,
  group: String,
}

impl DynTester {
  /// Registers a test, returning a reference to the test for further
  /// configuration.
  pub fn test<T: Termination, F: FnOnce() -> T + Send + 'static>(
    &mut self,
    name: impl Into<Name>,
    f: F,
  ) -> &mut DynTest {
    let index = self.tests.len();
    self.tests.push(DynTest(TestDescAndFn {
      desc: TestDesc {
        name: TestName::DynTestName(format!("{}{}", self.group, name.into().0)),
        ignore: false,
        ignore_message: None,
        source_file: self.file,
        start_line: self.line,
        start_col: self.col,
        end_line: self.line,
        end_col: self.col,
        should_panic: test::ShouldPanic::No,
        compile_fail: false,
        no_run: false,
        test_type: TestType::IntegrationTest,
      },
      testfn: TestFn::DynTestFn(Box::new(|| test::assert_test_result(f()))),
    }));
    &mut self.tests[index]
  }

  /// Groups a set of tests, akin to a module of tests.
  pub fn group(&mut self, name: impl Into<Name>, f: impl FnOnce(&mut Self)) {
    let len = self.group.len();
    self.group.push_str(&name.into().0);
    self.group.push_str("::");
    f(self);
    self.group.truncate(len);
  }

  /// Resolves a path relative to the package's manifest directory.
  pub fn resolve(&self, path: impl AsRef<Path>) -> PathBuf {
    Path::join(self.manifest.as_ref(), path)
  }

  /// Globs for files, relative to the package's manifest directory.
  ///
  /// Returns an iterator of pretty names and absolute paths.
  #[cfg(feature = "glob")]
  pub fn glob(&self, pattern: &str) -> impl Iterator<Item = (Name, PathBuf)> {
    self.glob_in(".", pattern)
  }

  /// Globs for files, relative to `base` (which is itself relative to the
  /// package's manifest directory).
  ///
  /// Returns an iterator of pretty names and absolute paths.
  #[cfg(feature = "glob")]
  pub fn glob_in(
    &self,
    base: impl AsRef<Path>,
    pattern: &str,
  ) -> impl Iterator<Item = (Name, PathBuf)> {
    use globset::GlobBuilder;
    use walkdir::WalkDir;

    let base = self.resolve(base);

    let walker = WalkDir::new(&base).follow_links(true);

    let glob = GlobBuilder::new(pattern)
      .case_insensitive(true)
      .literal_separator(true)
      .build()
      .unwrap()
      .compile_matcher();

    walker.into_iter().filter_map(move |file| {
      let file = file.unwrap();
      let path = file.path();
      let relative_path = path.strip_prefix(&base).ok()?;
      glob.is_match(relative_path).then(|| (relative_path.into(), path.to_owned()))
    })
  }
}

/// A dynamically registered test that can be configured further.
#[repr(transparent)]
pub struct DynTest(TestDescAndFn);

impl DynTest {
  /// Configure whether or not this test should be ignored; this is the analogue
  /// of `#[ignore]`.
  ///
  /// ```rust,no_run
  /// # let test: &mut dyntest::DynTest = unreachable!();
  /// test.ignore(true);     // #[ignore]
  /// test.ignore("reason"); // #[ignore = "reason"]
  /// ```
  pub fn ignore(&mut self, ignore: impl Into<Ignore>) -> &mut Self {
    let ignore = ignore.into();
    self.0.desc.ignore = ignore.ignore();
    self.0.desc.ignore_message = ignore.message();
    self
  }

  /// Configure whether or not this test is supposed to panic; this is the
  /// analogue of `#[should_panic]`.
  ///
  /// ```rust,no_run
  /// # let test: &mut dyntest::DynTest = unreachable!();
  /// test.should_panic(true);      // #[should_panic]
  /// test.should_panic("message"); // #[should_panic(expected = "message")]
  /// ```
  pub fn should_panic(&mut self, should_panic: impl Into<ShouldPanic>) -> &mut Self {
    self.0.desc.should_panic = should_panic.into().0;
    self
  }
}

/// A test name, or a fragment thereof.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(Cow<'static, str>);

/// Represents the value, if any, of a `#[ignore]` directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ignore {
  NoIgnore,
  Ignore(Option<&'static str>),
}

/// Represents the value, if any, of a `#[should_panic]` directive.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShouldPanic(test::ShouldPanic);

#[allow(non_snake_case, non_upper_case_globals)]
impl ShouldPanic {
  pub const No: ShouldPanic = ShouldPanic(test::ShouldPanic::No);
  pub const Yes: ShouldPanic = ShouldPanic(test::ShouldPanic::Yes);
  pub fn YesWithMessage(message: &'static str) -> Self {
    ShouldPanic(test::ShouldPanic::YesWithMessage(message))
  }
}

impl From<&'static str> for Name {
  fn from(value: &'static str) -> Self {
    Name(Cow::Borrowed(value))
  }
}

impl From<String> for Name {
  fn from(value: String) -> Self {
    Name(Cow::Owned(value))
  }
}

impl From<&Path> for Name {
  fn from(value: &Path) -> Self {
    Name(Cow::Owned(
      value
        .with_extension("")
        .components()
        .map(|x| x.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("::"),
    ))
  }
}

impl Ignore {
  pub fn ignore(&self) -> bool {
    matches!(self, Ignore::Ignore(_))
  }
  pub fn message(&self) -> Option<&'static str> {
    match *self {
      Ignore::NoIgnore => None,
      Ignore::Ignore(message) => message,
    }
  }
}

impl From<bool> for Ignore {
  fn from(value: bool) -> Self {
    if value { Ignore::Ignore(None) } else { Ignore::NoIgnore }
  }
}

impl From<&'static str> for Ignore {
  fn from(value: &'static str) -> Self {
    Ignore::Ignore(Some(value))
  }
}

impl From<String> for Ignore {
  fn from(value: String) -> Self {
    leak_string(value).into()
  }
}

impl From<bool> for ShouldPanic {
  fn from(value: bool) -> Self {
    if value { ShouldPanic::Yes } else { ShouldPanic::No }
  }
}

impl From<&'static str> for ShouldPanic {
  fn from(value: &'static str) -> Self {
    ShouldPanic::YesWithMessage(value)
  }
}

impl From<String> for ShouldPanic {
  fn from(value: String) -> Self {
    leak_string(value).into()
  }
}

fn leak_string(string: String) -> &'static str {
  Box::leak(string.into_boxed_str())
}
