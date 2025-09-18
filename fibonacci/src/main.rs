fn main() {
    println!("Fibonacci number");
    println!("Input numbers of terms");

    let mut numbers_of_terms = String::new();

    std::io::stdin()
        .read_line(&mut numbers_of_terms)
        .expect("Failed to read line");

    let numbers_of_terms: i32 = numbers_of_terms
        .trim()
        .parse()
        .expect("Failed to parse number");

    let value = fibonacci(numbers_of_terms);
    println!("value fibonacci is {value}");
}

fn fibonacci(n: i32) -> i32 {
    let mut previous = 0;
    let mut next = 1;

    for _i in 2..n {
        println!("{next}");
        let temp = next;
        next = previous + next;
        previous = temp;
    }

    next
}
