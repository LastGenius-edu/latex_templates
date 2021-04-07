fn main() {
    println!("{:?}", first_word("word1 word2"))
}

fn first_word<'a>(s: &'a str) -> Option<&'a str> {
    s.split(' ').next()
}
