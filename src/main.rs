mod built_in;

extern crate nom;

use std::io::{self, Write};
use std::collections::HashMap;
use types::{SpecialForm, Expression, Atom, EvalResult, Environment};

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
        let mut special_forms = HashMap::<String, SpecialForm>::new();
        let mut built_ins = HashMap::<String, fn(Vec<Expression>) -> EvalResult>::new();
        let mut vars = HashMap::<String, Expression>::new();
        built_ins.insert("+".to_string(), built_in::sum);
        built_ins.insert("*".to_string(), built_in::mul);
        built_ins.insert("/".to_string(), built_in::div);
        built_ins.insert("neg".to_string(), built_in::neg);
        special_forms.insert("quote".to_string(), SpecialForm::Quote);
        special_forms.insert("if".to_string(), SpecialForm::If);
        special_forms.insert("fn".to_string(), SpecialForm::Fn);
        vars.insert("variavel-qualquer".to_string(), Expression::At(Atom::Int(11)));

        let immut_built_ins = special_forms.clone();

        let env = Environment {
            special_forms: special_forms,
            built_in_fns: built_ins,
            vars: vars
        };

        /* Print user input line (just the parsed tree for now) */
        /* TODO:  Do a more user-friendly print. Either implement the Debug trait by hand or handle it otherwise */
        match parsed_input_expression {
            Ok((_, expr)) => {
                println!("eval: {}", eval::eval(expr, &env));
            }
            Err(_) => {
                println!("Fuck you, boah")
            }
        }
    }
}
