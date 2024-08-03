Day 2

On the second day of learning Rust, I learned about: Scalar Data Types

**Unsigned Integer**

Unsigned integers represent non-negative values only. They come in various sizes:

    u8: 8-bit unsigned integer
    u16: 16-bit unsigned integer
    u32: 32-bit unsigned integer
    u64: 64-bit unsigned integer
    u128: 128-bit unsigned integer

Example:

```rust
let unsigned_num: u8 = 5;
```

**Signed Integer**

Signed integers can represent both negative and positive values. They come in various sizes:

    i8: 8-bit signed integer
    i16: 16-bit signed integer
    i32: 32-bit signed integer
    i64: 64-bit signed integer
    i128: 128-bit signed integer

Example:


```rust
let signed_num = 5; // Defaults to i32 if not specified
```

**Floating Point Number**

Floating point numbers represent numbers with fractional parts. They come in two sizes:

    f32: 32-bit floating point number
    f64: 64-bit floating point number

Example:

```rust
let float_num: f32 = 5.0;
```

**Platform Specific Integers**

These types adjust to the size of the platform's architecture:

    usize: Unsigned integer whose size is dependent on the platform (32-bit on a 32-bit system, 64-bit on a 64-bit system)
    isize: Signed integer whose size is dependent on the platform (32-bit on a 32-bit system, 64-bit on a 64-bit system)

Example:


```rust
let arch_1: usize = 5;
let arch_2: isize = 5;
```

**Character**

Characters in Rust are represented by the char type. They can represent a single Unicode scalar value.

Example:

```rust
let character = 'a';
```

**Boolean**

Boolean values in Rust are represented by the bool type and can be either true or false.

Example:

```rust
let b = true;
```

**Type Aliasing**

Type aliasing allows you to create a new name for an existing type. This can make your code more readable.

Example:

```rust
type Age = u8;
let my_age: Age = 23;
```

**Type Conversion**

Rust does not automatically convert between different types. Instead, you must explicitly convert using the as keyword.

Example:

```rust
let a = 10;
let b = a as f64;
```

### Compound Data Types
**&str and String**

    &str: A string slice, which is an immutable reference to a string.
    String: A growable, mutable string type.

Example:

```rust
let fixed_str = "Fixed length string";
let mut flexible_string = String::from("This string will grow.");
flexible_string.push('s');
```

**Arrays**

Arrays are fixed-size collections of elements of the same type.

Example:



```rust
let mut array_1 = [3, 5, 6, 3, 6, 6];
let num = array_1[3]; // Accessing an element

println!("{:?}", array_1); // Prints the array
let array_2 = [0; 10]; // An array of ten zeros
```

**Vectors**

Vectors are resizable arrays. They are defined using the Vec type.

Example:

```rust
let vec_1 = vec![4, 3, 2, 3, 2];
let num = vec_1[3]; // Accessing an element
```

**Tuples**

Tuples are fixed-size collections of elements of different types.

Example:

```rust
let my_info = ("Salary", 4000, "Age", 49.2);
let salary_value = my_info.3; // Accessing an element by index
let (salary, salary_value, age, age_value) = my_info; // Destructuring
```

**Unit Type**

The unit type () is a type that has only one value, (). It is used when a value is not needed.

Example:

```rust
let unit = ();
```
