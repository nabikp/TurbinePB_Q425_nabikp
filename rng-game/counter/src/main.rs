use std::io;

fn main() {
    let mut count = 0;
    loop {
        println!("Count is {}", count);
        println!("Press Enter to increment or type 'q' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            break;
        }
        count += 1;
    }
}
