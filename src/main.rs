mod built_in;

extern crate nom;

use std::io::{self, Write};

mod parser;
mod eval;
mod types;

fn main() {
    /* Initial prompt and shit */
    println!("Lispy Version 0.0.0.0.1");
    println!("Press Ctrl+c to Exit\n");

    let mut input_history = Vec::new();

    /* Construct built-in function table
           TODO: Move construction to built_in module itself */
    let mut env = built_in::initial_env();

    loop {
        print!("lispy_rusty>");
        io::stdout().flush().unwrap(); // Flush to write prompt before getting input

        let mut x = String::new();

        /* Read user input line */
        /* TODO: voltar o cursor no REPL. Ngm merece ^[[D */
        io::stdin().read_line(&mut x).expect("Failed to read line");

        /* Record input */
        input_history.push(x.clone());

        /*  TODO; UNDERSTAND: Why does "let parsed_input_expression = parser::parse(&x.to_owned());" not work? */
        /*  Parse input into expression tree */
        let to_parse = x.to_owned();
        let parsed_input_expression = parser::parse(&to_parse);

        /* Print user input line (just the parsed tree for now) */
        /* TODO:  Do a more user-friendly print. Either implement the Debug trait by hand or handle it otherwise */
        match parsed_input_expression {
            Ok((_, expr)) => {
                println!("eval: {}", eval::eval(expr, &mut env));
            }
            Err(parser_error) => {
                println!("Fuck you, boah: {:?}", parser_error)
            }
        }
    }
}
