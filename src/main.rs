use std:: env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    println!("{}", input.chars().rev().collect::<String>());
}
