error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> src/main.rs:2:29
  |
2 |     format!("{} + {} = {}", a, b, a + b)
  |                             ^ `T` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: required by `std::fmt::Display::fmt`
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
  |
1 | fn adder<T: std::ops::Add<Output = T> + std::fmt::Display>(a: T, b: T) -> String {
  |                                       ^^^^^^^^^^^^^^^^^^^
