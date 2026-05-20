//full working CLI Calculator

// Import standard input/output library
use std::io;
fn main() {
    println!("===RUST CALCULATOR");

     // Create a mutable String to store user input
    let mut input = String::new();

    // ---- FIRST NUMBER INPUT ---- 
    println!("Enter First Number");

    // Read user input from stdin and store in 'input'
    io::stdin().read_line(&mut input).expect("FAILED TO READ");

     // Convert input (String) → f64 after trimming whitespace
    let num1: f64 = input.trim().parse().expect("INVALID NUMBER");

    // Clear input buffer so we can reuse it
    input.clear();

     // ---- OPERATOR INPUT ----
    println!("ENTER OPERATOR (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("FAILED TO READ");

    // Convert input to String (owned value)
    let operator = input.trim().to_string();

    input.clear();

     // ---- SECOND NUMBER INPUT ----
    println!("ENTER SECOND NUMBER:");
    io::stdin().read_line(&mut input).expect("FAILED TO READ");
    let num2: f64 = input.trim().parse().expect("INVALID NUMBER");

     // ---- CALCULATION LOGIC ----
    // Use match for pattern-based control flow
    let result = match operator.as_str() {"+" => num1 + num2,
"-" => num1 -num2,
"*" => num1 * num2,
"/" => {
     // Prevent division by zero
    if num2 ==0.0 {
        println!("ANNOT DIVIDE BY ZERO");
        return;
    }
    num1 / num2
}
_ => {
     // Catch invalid operators
    println!("INVALID OPERATOR");
    return;
}
};
// ---- OUTPUT RESULT ----
println!("RESLT: {}", result)
}