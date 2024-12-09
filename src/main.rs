use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. We take args
    // 2. We process the args and get the appropriate value needed
    // 3. Perform operation based on that

    println!("Args: {args:#?}");
}
