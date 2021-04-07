// Iteration
for (key, value) in &grades {
    println!("{}: {}", key, value);
}

// Creation from an iterator
let grades = [("Student1", 100), ("Student2", 90)]
        .iter()
        .cloned()
        .collect::<HashMap<&str, i32>>();
