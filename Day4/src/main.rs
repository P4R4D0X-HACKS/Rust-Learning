// Ownership

fn main() {
    let s1 = String::from("world"); // Strings are heap allocated
    let s2 = s1; // s2 has become the owner of the world
    println!("{s2}");

    {
        let s3 = s2;
        println!("{s3}");
    } // s3 is removed from stack itself

    let s4 = String::from("Hello");
    let s5 = s4.clone(); // this is how we can clone it and have 2 owners
    println!("{s4}, {s5}");


    let x = 5;
    let y = x;
    println!("{y}"); // Data types like int, char, float, bool have their own owner as they all reside in stack

    let vec_1 = vec![5, 4, 3];
    take_ownership(vec_1.clone());
    println!("The vec_1 is {:?}", vec_1);

    let vec_2 = give_ownership();
    println!("The Vec_2 is {:?}", vec_2);

    // Borrowing

    let mut vec_3 = vec![4, 5, 6];
    let a = &vec_3;
    let b = &vec_3;
    println!("Ref1 : {:?}, ref2: {:?}", a, b);
    let c = &mut vec_3;
    println!("{:?}", c);
    borrows_vec(c);



}


fn take_ownership(vec: Vec<i32>) {
    println!("The output is {:?}", vec)
}

fn give_ownership() -> Vec<i32> {
    vec![1, 2, 3]
}

// Borrowing

fn borrows_vec(vec: &Vec<i32>){
    println!("Vec is: {:?}", vec)
}