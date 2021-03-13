let x = Some("value");
let y = None;

assert_eq!(x.unwrap(), "value");
y.unwrap(); // Will panic

assert_eq!(x.unwrap_or("default"), "value");
assert_eq!(y.unwrap_or("default"), "default");
