pub fn main() {
    println!("Hello, world!");

    let x: i8 = 10;
    println!("{}", x);

    println!("{}", add(10, 20));

    let p: String = String::from("Hello world!");
    println!("{}", p);

    let age: i32 = 21;
    if age >= 18 {
        println!("You are eligible for Voting");
    } else {
        println!("You are not eligible for Voting.")
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
