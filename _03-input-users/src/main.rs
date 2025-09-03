fn main() {
    let mut name: String = String::new();
    let mut age_for_keyboard: String = String::new();
    println!("Input your name:");
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Input your age:");
    std::io::stdin().read_line(&mut age_for_keyboard).unwrap();
    let age: u8 = age_for_keyboard.trim().parse().unwrap();
    println!("Hello, welcome {name}, your age is {age_for_keyboard}");
}
