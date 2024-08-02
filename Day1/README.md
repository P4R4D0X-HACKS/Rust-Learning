## Day 1

**In the first day of learning Rust I learned about:**

### Variables

#### How they are defined

In Rust, variables are defined using the `let` keyword. By default, variables are immutable, meaning their value cannot be changed once assigned. To define a variable, you use the following syntax:

```rust
let a = 5;
```
You can also specify the type explicitly, though Rust can usually infer it:

`let a: i32 = 5;`

Rust supports various types such as:

    i32 (32-bit signed integer)
    f32 (32-bit floating point)
    i16 (16-bit signed integer)

### Mutability

#### If you want to define a variable whose value can be changed, you use the mut keyword:


```rust
let mut b = 10;
b = 12;
println!("b is {b}"); // b is 12
```

### Scope

Variables in Rust have a scope, which is the region of the code where a variable is valid. A variable introduced inside a block (enclosed in {}) is not accessible outside of it.

```rust
let c = 20;
{
let c = 21;
println!("inner scope c = {c}"); // inner scope c = 21
}
println!("outer scope c = {c}"); // outer scope c = 20
```

### Shadowing

Rust allows you to declare a new variable with the same name as a previous variable. The new variable shadows the previous one:

```rust
let d = 33;
let d = 23;
println!("d is {d}"); // d is 23
```

### Constants

Constants in Rust are defined using the const keyword and must have a type annotation. Constants are always immutable and can be defined outside of functions.

```rust
const MAXIMUM: i32 = 3;
println!("{MAXIMUM}"); // 3
```