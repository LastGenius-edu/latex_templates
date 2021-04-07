fn main() {
    println!("{:?}", first_word("word1 word2"))
}

fn first_word(s: &str) -> Option<&str> {
    s.split(' ').next()
}
