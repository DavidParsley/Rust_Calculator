use std::io;
fn main() {
    println!("Hello 👋 from Davids Frist Rust Calculator!");
    println!("I was created on September 3rd 2025 👨‍🍼");

    println!("Whats Your Name 🕵️ ?");
    let mut user_name = String::new();
    io::stdin()
        .read_line(&mut user_name)
        .expect("Failed to read line");
    let user_name = user_name.trim();
    println!("Hello {} 🚀! Lets do some Maths 🫵 !", user_name);

    let mut input_a = String::new();

    println!("Please enter the first number (e.g.,1️⃣, 2️⃣, 3️⃣, 4️⃣)");
    io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read line");
    println!("You entered: {}", input_a);

    let mut input_op = String::new();

    println!("Please Enter an operator (e.g.,➕, ➖, *, /):");
    io::stdin()
        .read_line(&mut input_op)
        .expect("Failed to read line");
    println!("You entered: {}", input_op);

    let mut input_b = String::new();

    println!("Please enter the second number (e.g.,5️⃣, 6️⃣, 7️⃣, 8️⃣)");
    io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read line");
    println!("You entered: {}", input_b);

    let a: f64 = input_a
        .trim()
        .parse::<f64>()
        .expect("Please type a number 🔢!");

    let op = input_op.trim();

    let b: f64 = input_b
        .trim()
        .parse::<f64>()
        .expect("Please type a number 🔢!");

    match op {
        "+" => println!("{} The Result is: {} 🥳", user_name, a + b),
        "-" => println!("{} The Result is: {} 🥳", user_name, a - b),
        "*" => println!("{} The Result is: {} 🥳", user_name, a * b),
        "/" => {
            if b != 0.0 {
                println!("{} The Result is: {} 🥳", user_name, a / b);
            } else {
                println!("{} there is an Error: You can't divide by 0 😬 !", user_name);
                return;
            }
        }
        _ => {
            println!("{} That's an Invalid operator 🧐 .", user_name);
            return;
        }
    }
}

