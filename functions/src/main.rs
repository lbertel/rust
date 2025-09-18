fn main() {
    // println!("Hello, world!");
    //
    // another_function(5, 'h');
    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The valur of y is {y}");
    let x = five();
    let y = plus_one(x);
    println!("The value of x  is: {x}");
    println!("The value of y  is: {y}");
}

// fn another_function(value: i32, unit_label: char) {
//     println!("The measurement value is: {value}{unit_label}");
// }

fn five() -> i32{
    5
}

fn plus_one(x:i32) -> i32{
    x + 1
}