fn main() {
    my_fn("This is my function");
    let str = "Function call with a variable";
    my_fn(str);
    // println!("{}", multiplication(5, 5));
    let answer = multiplication(5, 5);
    my_ans(answer);
    let answer_add = addition(5,6,7,8);
    // println!("{answer}")
    my_ans(answer_add);
    let (addition, multiplication, subtraction, division) = basic_math(10, 10);
    println!("{addition}, {multiplication}, {subtraction}, {division}")


}

fn my_ans(ans: i32) {
    println!("{ans}")
}
fn my_fn(s: &str) {
    println!("{s}");
}

fn multiplication (a: i32, b: i32) -> i32 {
    println!("Multiplying");
    a * b
}

fn addition(a: i32, b: i32, c: i32, d: i32) -> i32 {
    println!("Addition");
    a + b + c + d
}

fn basic_math(a: i32, b: i32) -> (i32, i32, i32, f64) {
    (a + b, a*b, a - b, (a / b) as f64)
}

// Conditionals

fn conditions() {
    let num = 40;
    if num < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is greater than or equal to 50")
    }

    // if else if ladder
    let marks = 95;
    let mut grade = 'A';
    if marks >= 90 {
        grade = 'A';
    } else if marks >=80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else {
        grade = 'F';
    };

    match marks { // let grade = match marks {
        90..=100 => grade = 'A', // 90..=100 => 'A',
        80..=89 => grade = 'B', // 80..=89 => 'B',
        70..=79 => grade = 'C', // 70..=79 => 'C',
        _ => grade = 'F', // _ => 'F' }
    }

}

fn  looping() {
    // 'outer: loop {
    //     println!("Simple Loop");
    //     break 'outer;
    // };
    //
    // let a = loop {
    //     break 5;
    // };

    let vec = vec![54, 34, 23, 53, 55];

    for i in vec{
        println!("{i}");
    };

    let mut num = 0;
    while num < 10 {
        num = num + 1;
    }
}

fn extra() {
    // The current line is a comment line
    // This is the second line of comment

    /* This is a
    multiple line
    comment.
     */

    /*
    \n : Newline character
    \t : Tab Space
    \r : Carriage Return
    \" : Double quote
    \\ : backward slash
     */

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read Input");
    let n: f64 = n.trim().parse().expect("invalid input");
    println!("{n}");

    let _number = 400;

    static WELCOME: &str = "Welcome to Rust";

}