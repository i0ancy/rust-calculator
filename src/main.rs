use std::io::{self, Write};

fn read_var(input: &mut String) {
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(input).expect("Failed to read");
}

fn main() {
    let valid_operators = String::from("+-/*");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("Please enter num1: ");
        read_var(&mut num1);
        let num1: f64 = num1.trim().parse().unwrap();
    
        print!("Please enter num2: ");
        read_var(&mut num2);
        let num2: f64 = num2.trim().parse().unwrap();
    
        print!("Please enter operator(+-*/): ");
        read_var(&mut operator);
        let operator: char = operator.trim().chars().next().unwrap();
    
        if !valid_operators.contains(operator) {
            println!("Invalid operator...");
            continue;
        }
    
        match operator {
            '+' => {
                println!("{} {} {} = {}", num1, operator, num2, num1+num2)
            },
            '-' => {
                println!("{} {} {} = {}", num1, operator, num2, num1-num2)
            },
            '*' => {
                println!("{} {} {} = {}", num1, operator, num2, num1*num2)
            },
            '/' => {
                println!("{} {} {} = {}", num1, operator, num2, num1/num2)
            },
            _ => {
                println!("Interesting error...")
            }
        }
    }
}
