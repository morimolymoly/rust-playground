pub fn take_ownership(s: String) {
    println!("take ownership {}", s);
}

pub fn not_take_ownership(s: &String) {
    println!("not take ownership {}", s);
}

pub fn not_take_ownership_but_mutable(s: &mut String) {
    s.push_str("mmmmm");
}

pub fn take_ownership_and_give(s: String) -> String {
    println!("take ownership and give{}", s);
    s
}

pub fn take_and_print<T> (var: &T)
where T: std::fmt::Display
{
    println!("take_and_print {}", var);
}
