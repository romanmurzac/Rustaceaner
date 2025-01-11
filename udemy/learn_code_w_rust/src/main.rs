fn color_to_number(color: &str) {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_ref(color: &str) {
    match color {
        "red" => 1,
        "green" => 2,
        "red" => 3,
        _ => 0,
    }
}

fn factorial_iter(number: i32) -> i32 {
    let mut product: i32 = 1;
    let mut count: i32 = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn factorial_rec(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial_rec(number - 1)
}

fn main() {
    println!("{}:{}", color_to_number("red"), color_to_number_ref("red"));
    println!("{}:{}", color_to_number("green"), color_to_number_ref("green"));
    println!("{}:{}", color_to_number("blue"), color_to_number_ref("blue"));
    println!("{}:{}", color_to_number("yellow"), color_to_number_ref("yellow"));

    println!("{}", factorial_iter(6));
    println!("{}", factorial_rec(6));
}

/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
 
Refactor the function above to use the `match` statement
instead of if, else if, and else.
 
Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
 
Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.
 
The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.
 
Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/