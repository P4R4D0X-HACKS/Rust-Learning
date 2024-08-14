## Day 4

**On the fourth day of learning Rust, I learned about:**

### Ownership

Rust's ownership model is one of its key features, ensuring memory safety without needing a garbage collector. Ownership is a set of rules that govern how a Rust program manages memory.

#### Moving Ownership
- When a variable in Rust is assigned to another variable, the ownership of the data is moved.
- For example, when `s1` is assigned to `s2`, `s1` no longer owns the data; `s2` becomes the owner.

Example:
```rust
fn main() {
    let s1 = String::from("world"); // Strings are heap allocated
    let s2 = s1; // s2 has become the owner of "world"
    println!("{s2}");
}
```

After the inner scope ends, s3 is no longer valid, and its memory is freed.
Cloning Data

If you want to keep the original data after transferring ownership, you can clone it.

Example:

```rust
fn main() {
    let s4 = String::from("Hello");
    let s5 = s4.clone(); // Cloning allows both s4 and s5 to own the data
    println!("{s4}, {s5}");
}
```

In this example, both s4 and s5 are valid and point to different memory locations containing the same data.
Ownership with Simple Data Types

    Simple data types like integers, floats, characters, and booleans are stored on the stack and have a copy by default.

Example:

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("{y}"); // x and y both own the value 5
}

```

Both x and y are valid because the value is copied instead of moved.
Functions and Ownership

When passing variables to functions, the ownership of the data can be transferred. If you want to keep the ownership, you need to clone the data.

Example:

```rust
fn main() {
    let vec_1 = vec![5, 4, 3];
    take_ownership(vec_1.clone()); // Cloning to keep ownership of vec_1
    println!("The vec_1 is {:?}", vec_1);

    let vec_2 = give_ownership(); // vec_2 takes ownership of the returned vector
    println!("The Vec_2 is {:?}", vec_2);
}

fn take_ownership(vec: Vec<i32>) {
    println!("The output is {:?}", vec)
}

fn give_ownership() -> Vec<i32> {
    vec![1, 2, 3]
}
```

In this example, vec_1 is cloned before being passed to take_ownership, so vec_1 remains valid after the function call.
Borrowing

Rust allows you to borrow a reference to a value without transferring ownership. Borrowing is done using references, which are denoted by &.
Immutable Borrowing

    Multiple references can borrow a value simultaneously as long as none of them modify the value.

Example:

```rust
fn main() {
    let mut vec_3 = vec![4, 5, 6];
    let a = &vec_3; // Immutable reference
    let b = &vec_3; // Another immutable reference
    println!("Ref1 : {:?}, ref2: {:?}", a, b);
}
```



Here, both a and b borrow vec_3 immutably, so they can coexist.
Mutable Borrowing

    You can borrow a value mutably using &mut. However, Rust enforces a rule that only one mutable reference can exist at a time.

Example:

```rust
fn main() {
    let mut vec_3 = vec![4, 5, 6];
    let c = &mut vec_3; // Mutable reference
    println!("{:?}", c);
    borrows_vec(c); // Passing the mutable reference to a function
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("Vec is: {:?}", vec)
}

```


In this example, c mutably borrows vec_3, and no other mutable or immutable reference can coexist with c.
Summary

In Day 4, I learned about Rust's ownership model, how it ensures memory safety, and how borrowing allows you to work with references without transferring ownership. The examples demonstrated moving ownership, cloning data, and borrowing both immutably and mutably.