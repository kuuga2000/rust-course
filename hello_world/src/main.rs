fn main() {
    println!("{}", with_static_return_int());
    println!("{}", with_param_int(20));
    println!("{}", with_param_string("Hello Rust!"));
    println!("{}", with_static_return_string())
}

fn with_static_return_int() -> i8 {
    return 20;
}

fn with_param_int(int: i8) -> i8 {
    return int*2;
}

fn with_param_string(text: &str) -> &str {
    return text;
}

fn with_static_return_string() -> &'static str {
    return "Hello Rust with Static String";
}