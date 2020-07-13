mod values;

extern crate nom;

use std::io::{self, Write};
use nom::lib::std::collections::HashMap;
use types::{ResolvedSymbol, SpecialForm};

mod parser;
mod eval;
mod types;

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
        let mut built_ins = HashMap::<String, ResolvedSymbol>::new();
        // built_ins.insert("+".to_string(), values::sum);
        // built_ins.insert("*".to_string(), values::mul);
        // built_ins.insert("/".to_string(), values::div);
        // built_ins.insert("neg".to_string(), values::neg);
        built_ins.insert("quote".to_string(), ResolvedSymbol::SpecialF(SpecialForm::Quote));
        built_ins.insert("if".to_string(), ResolvedSymbol::SpecialF(SpecialForm::If));

        let immut_built_ins = built_ins.clone();

        /* Print user input line (just the parsed tree for now) */
        /* TODO:  Do a more user-friendly print. Either implement the Debug trait by hand or handle it otherwise */
        match parsed_input_expression {
            Ok((_, expr)) => {
                println!("eval: {}", eval::eval(expr, &immut_built_ins));
            }
            Err(_) => {
                println!("Fuck you, boah")
            }
        }
    }
}
