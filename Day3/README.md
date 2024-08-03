## Day 3

**On the third day of learning Rust, I learned about:**

### Functions

Functions in Rust are defined using the `fn` keyword. Functions can accept parameters and return a value.

#### Function Definition and Calls
Example:
```rust
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
```
### Functions with Parameters

Example:

```rust
fn my_fn(s: &str) {
    println!("{s}");
}
```

This function takes a string slice (&str) as a parameter and prints it.
Functions with Return Values

Example:

```rust
fn multiplication(a: i32, b: i32) -> i32 {
    println!("Multiplying");
    a * b
}
```

This function takes two i32 parameters, prints a message, and returns their product.
Functions with Multiple Parameters

Example:

```rust
fn addition(a: i32, b: i32, c: i32, d: i32) -> i32 {
    println!("Addition");
    a + b + c + d
}
```

This function takes four i32 parameters, prints a message, and returns their sum.
Functions with Multiple Return Values

Example:

```rust
fn basic_math(a: i32, b: i32) -> (i32, i32, i32, f64) {
    (a + b, a * b, a - b, (a / b) as f64)
}
```

This function takes two i32 parameters and returns a tuple containing the sum, product, difference, and division (as f64) of the parameters.

### Conditionals

Rust provides conditionals using the if keyword. You can also use else if for multiple conditions and else for the default case.

Example:

```rust
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

    // Using match statement for conditions
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    }
}

```


### Looping

Rust provides multiple ways to loop through code: loop, while, and for.
Infinite Loop

Example:

```rust
fn looping() {
    'outer: loop {
        println!("Simple Loop");
        break 'outer;
    };
}

Using loop to Return a Value

Example:

rust

fn looping() {
    let a = loop {
        break 5;
    };
}
```

### For Loop

Example:

```rust
fn looping() {
    let vec = vec![54, 34, 23, 53, 55];
    for i in vec {
        println!("{i}");
    }
}

```
### While Loop

Example:

```rust
fn looping() {
    let mut num = 0;
    while num < 10 {
        num = num + 1;
    }
}
```

## Extra Features

### Comments

Rust supports single-line comments using // and multi-line comments using /* */.

Example:

```rust
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
}
```


### Reading User Input

Example:

```rust
fn extra() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read Input");
    let n: f64 = n.trim().parse().expect("invalid input");
    println!("{n}");
}
```

### Static Variables

Static variables are defined using the static keyword. They have a 'static lifetime.

Example:

```rust
fn extra() {
    static WELCOME: &str = "Welcome to Rust";
}
```
