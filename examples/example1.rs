use std::env;

use bt_math::evaluate_expression;

fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Print the number of arguments received
    println!("Number of arguments: {}", args.len());

    // Iterate over the arguments and print them
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }

    // Example: Accessing specific arguments. Each argument is a math expression ("2*5/3" "sin(45)").
    if args.len() > 1 {
        let expressions = args[1..].to_vec();

        for expression in expressions {
            match evaluate_expression(&expression) {
                Ok(result)  => println!("Result of '{}'= {}", expression, result),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}