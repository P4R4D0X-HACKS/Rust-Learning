fn main(){
    // Unsigned integer
    let unsigned_num:u8 = 5; // u16, u32, u64, u128

    // Signed integer
    let signed_num = 5; // i32, i8, i64, i128

    // Floating point number
    let float_num:f32 = 5.0; // f64

    // Platform specific integers
    let arch_1: usize = 5;
    let arch_2: isize = 5;

    // Character
    let char = 'a';

    // Boolean
    let b = true;

    // Type aliasing
    type Age = u8;
    let my_age: Age = 23;

    // Type Conversion
    let a = 10;
    let b = a as f64;
}

fn compound_datatypes(){
    // &str and string
    let fixed_str = "Fixed length string";
    let mut flexible_string = String::from("This string will grow.");
    flexible_string.push('s');

    // Arrays
    let mut array_1 = [3,5,6,3,6,6];
    let num = array_1[3];

    println!("{:?}", array_1);
    let array_2 = [0; 10];

    // Vectors
    let vec_1 = vec![4,3,2,3,2];
    let num = vec_1[3];

    // Tuples
    let my_info = ("Salary", 4000, "Age", 49.2);
    let salary_value = my_info.3;
    let (salary, salary_value, age, age_value) = my_info;

    let unit = ();

}

