use std::io;

//Program asks for 2 positive  integers, then adds them together
//If you give a number over the max int, it changes it to 0

fn main() {
    println!("Enter 2 positive integers");
 
    let a = string_to_u32(read_string());
    let b = string_to_u32(read_string());
    
    //println!("{}", string_to_u32(read_string()));
    //Legal, but doesn't let me do the sum later

    println!("The numbers {} and {} were input", a, b);
    println!("The sum is {}", a + b);
}

//Takes in a string from the user and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 0
fn string_to_u32(x: String) -> u32 {
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}

