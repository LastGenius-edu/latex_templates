loop {
    println!("Oh no, here we go again...");
}

let result = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
};
