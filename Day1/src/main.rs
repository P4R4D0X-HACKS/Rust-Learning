fn main() {
    // Definition
    let a = 5;
    println!("a is {a}");
    // Mutability
    let mut b = 10;
    b = 12;
    println!("b is {b}");
    // Scope
    let c = 20;
    {
        let c = 21;
        println!("inner scope c = {c}")
    }
    println!("outer scope c = {c}");
    // Shadowing
    let d = 33;
    let d = 23;
    println!("d is {d}");
    // Constant
    const MAXIMUM:i32 = 3;
    println!("{MAXIMUM}")
}