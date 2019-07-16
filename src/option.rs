pub fn UnwrapOption(op: Option<u64>) -> u64 {
    match op {
        None => 0,
        Some(n) => n,
    }
}
