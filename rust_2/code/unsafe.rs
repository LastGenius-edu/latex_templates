fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx < arr.len() {
        // SAFETY: I am so smart that I know this can never cause errors
        // I don't know why do I even bother with safe Rust.
        unsafe { Some(*arr.get_unchecked(idx)) }
    } else {
        None
    }
}
