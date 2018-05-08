Version 0.2.2 (2018-05-08)
=========================

- Propperly handle `until_acceptable` with `.accept()` and add test
- Add an example of using `.accept()`

Version 0.2.1 (2017-11-09)
=========================

Documentation
-------------
- Add links to release and running release docs in README
- Add initial module level documentation 
- Add initial public interface documentation
- Updated examples

CI
--
- Require nightly and beta to pass tests
- Run rustfmt and clippy on nightly

Misc
----
- Ran rustfmt
- Ran clippy 

Version 0.2.0 (2017-11-05)
=========================

Public Interface
----------------

- The `acceptable()` method now accepts `Vec<&str>` instead of `&[String]`.
  Example:
  ```rust
  fn main() {
	let question = Question::new("Continue?").acceptable(vec!["y", "n"]);
  }
  ```
- Added setter for providing `clarification`.
  Example:
  ```rust
  fn main() {
	let clarification = "Please enter one of (yes/no)";
	let question = Question::new("Continue?").clarification(clarification);
  }
  ```

Bug Fixes
---------
- Flush `stdio` before getting user input
- Strip newlines from user response
- Add a space after the question prompt
- Add a space between question and defaults
- If a default answer exists return it when empty string is given

CI
--
- Added CI for Linux
- Added CI for Windows
- Added Code Coverage
- Added tests for entire public interface

Documentation
-------------
- Added CHANGELOG
- Added CONTRIBUTING
- Completed README

Misc
----
- Refactor to support dependency injection replacing `Stdin` and `Stdout` in tests.


Version 0.1.0 (2017-11-03)
=========================

- Initial release
- Do not use was essentially just scaffolding
