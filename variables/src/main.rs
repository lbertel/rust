use std::io;

fn main() {
    //--------mutable variables------
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    //-----------shadowing variables-----------
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    //
    // println!("The value of x is: {x}");

    //---------------tuples---------------
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    // let (x, y, z) = tup;
    //
    // println!("x: {}, y: {}, z: {}", x, y, z);
    //
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;
    //
    // println!("x: {}, y: {}, z: {}", five_hundred, six_point_four, one);

    //-------------------arrays-------------------
    // let a = [1, 2, 3, 4, 5];
    //
    // println!("Please enter an array index.");
    //
    // let mut index = String::new();
    //
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line.");
    //
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number.");
    //
    // let element = a[index];
    //
    // println!("The value of element at index {index} is: {element}");

    //-------functions---------------
    println!("Hello world!");

    another_function();
}

fn another_function() {
    println!("Another function")
}
