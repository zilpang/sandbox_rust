use std::io;

fn main() {
    another_function();

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn another_function() {
    let y = {
        let x = 3;
        x + 1 // expression
    };

    println!("The value of y is: {}", y);
}
