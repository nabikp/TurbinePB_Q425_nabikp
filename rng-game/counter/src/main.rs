use std::io;

fn main() {
    let mut count = 0;

    println!("Welcome to the Rust counter!");
    println!("Hit Enter to increase the count, or type 'q' to exit.");

    loop {
        println!("Current count: {}", count);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("Goodbye! Final count: {}", count);
            break;
        }

        count += 1;
    }
}