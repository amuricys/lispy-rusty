extern crate nom;

use std::io::{self, Write};
use nom::lib::std::collections::HashMap;

mod parser;
mod built_in;

fn main() {
    /* Initial prompt and shit */
    println!("Lispy Version 0.0.0.0.1");
    println!("Press Ctrl+c to Exit\n");

    let mut input_history = Vec::new();
    loop {
        print!("lispy_rusty>");
        io::stdout().flush().unwrap(); // Flush to write prompt before getting input

        let mut x = String::new();

        /* Read user input line */
        io::stdin().read_line(&mut x).expect("Failed to read line");

        /* Record input */
        input_history.push(x.clone());

        /*  TOUNDERSTAND: Why does "let parsed_input_expression = parser::parse(&x.to_owned());" not work?
            Parse input into expression tree */
        let to_parse = x.to_owned();
        let parsed_input_expression = parser::parse(&to_parse);

        /* Construct built-in function table 
           TODO: Move construction to built_in module itself */
        let mut built_ins = HashMap::<String, fn(i64, i64) -> i64>::new();
        built_ins.insert("+".to_string(), built_in::sum);
        built_ins.insert("-".to_string(), built_in::sub);
        built_ins.insert("*".to_string(), built_in::mul);
        built_ins.insert("/".to_string(), built_in::div);

        let immut_built_ins = built_ins.clone();

        /* Print user input line (just the parsed tree for now) */
        match parsed_input_expression {
            Ok((_, expr)) => {
                println!("{}", parser::eval(expr, &immut_built_ins));
            }
            Err(_) => {
                println!("Fuck you, boah")
            }
        }
    }
}
