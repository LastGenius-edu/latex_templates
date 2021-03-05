// Will create a str 
let str = "Hello! I'm a str";

// Will create a String
let string = String::from("Hello! I'm a str");

// Will create a slice into string
// Be careful! Rust will panic if you attempt to
// slice a string inside a character
let slice = &s[1..4];
