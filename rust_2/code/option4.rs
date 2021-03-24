let res = divide(2.0, 3.0);

// if `let` destructures `res` into
// `Some(i)`, evaluate the block (`{}`).
if let Some(x) = res {
    println!("Result: {}", x);
}
