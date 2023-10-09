fn main() {
    let mut line1 = String::new();
    println!("Enter first number :");
    std::io::stdin().read_line(&mut line1);
    line1.pop();

    let mut line2 = String::new();
    println!("Choose Operation from +-*/ :");
    std::io::stdin().read_line(&mut line2);
    line2.pop();

    let mut line3 = String::new();
    println!("Enter second number :");
    std::io::stdin().read_line(&mut line3);
    line3.pop();

    let number1 : f64;
    let number2 : f64;

    match line1.parse::<f64>(){
        Ok(value) => {
            number1 = value;
        }
        Err(error_message) => {
            println!("First number can't be parsed {}", error_message);
            return;
        }
    }

    match line3.parse::<f64>(){
        Ok(value) => {
            number2 = value;
        }
        Err(error_message) => {
            println!("Second number can't be parsed {}", error_message);
            return;
        }
    }

    let operation: Option<Operation> = match line2.as_str() {
        "+" => {
            Some(Operation::Add(number1, number2))
        }
        "-" => {
            Some(Operation::Subtract(number1, number2))
        }
        "*" => {
            Some(Operation::Multiply(number1, number2))
        }
        "/" => {
            Some(Operation::Divide(number1, number2))
        }
        _ => {
            None
        }
    };
    match operation {
        Some(op) => {
            let result = calculate(op);
            match result {
                Ok(value) => {
                    println!("The result is : { }", value);
                }
                Err(error_message) => {
                    println!("This operation can't be done because { }", error_message);
                }
            }
        }
        None => {
            println!("Not a valid input for operation");
        }
    }



}

enum Operation {
    Add(f64, f64 ),
    Subtract(f64, f64 ),
    Multiply(f64, f64 ),
    Divide(f64, f64 )
}
fn calculate(op: Operation) -> Result<f64,String> {
    match op {
        Operation::Add(val1, val2) => {
            Ok(val1 + val2)
        }
        Operation::Subtract(val1, val2) => {
            Ok(val1 - val2)
        }
        Operation::Multiply(val1, val2) => {
            Ok(val1 * val2)
        }
        Operation::Divide(val1, val2) => {
            if val2 == 0.0 {
                Err(String::from("Division by zero is not allowed."))
            }
            else {
                Ok(val1 / val2)
            }
        }
    }
}
