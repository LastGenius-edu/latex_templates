let res = divide(2.0, 3.0);

// while `let` destructures `res` into
// `Some(i)`, evaluate the block (`{}`). Else `break`.
while let Some(x) = res {
    println!("Result: {}", x);
}
