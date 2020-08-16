extern crate nom;

use std::io::{self, Write};
use nom::AsBytes;
use types::{Environment, Expression};
use eval::LocalVars;

mod parser;
mod eval;
mod types;
mod util;
mod built_in;

fn repl(mut env: &mut Environment, local_env: LocalVars ) {
    /* Initial prompt and shit */
    println!("Lispy Version 0.0.0.1.1");
    println!("Press Ctrl+c to Exit\n");

    let mut input_history = Vec::new();

    loop {
        print!("lispy_rusty>");
        io::stdout().flush().unwrap(); // Flush to write prompt before getting input

        let mut x = String::new();

        /* (R)ead user input line */
        /* TODO: voltar o cursor no REPL. Ngm merece ^[[D */
        io::stdin().read_line(&mut x).expect("Failed to read line");

        /* record input */
        input_history.push(x.clone());

        /*  TODO; UNDERSTAND: Why does "let parsed_input_expression = parser::parse(&x.to_owned());" not work? */
        /*  parse input into expression tree */
        let to_parse = x.to_owned();
        let parsed_input_expression = parser::parse(&to_parse);

        match parsed_input_expression {
            Ok((_, expr)) => {
                /* (E)valuate the user input*/
                let result = eval::eval(expr, &mut env, &local_env);
                /* (P)rint result of evaluation TODO: This print should look better. Perhaps implement Debug by hand*/
                println!("{}", result);
            }
            Err(parser_error) => {
                println!("Fuck you, boah: {:?}", parser_error) // TODO: Parser errors could be turned into 'syntax errors'
            }
        }
        /* (L)oop :point_up: */
    }
}

fn load_main_namespace(mut env: &mut Environment, local_env: LocalVars, filepath: &str) {
    let contents = std::fs::read(&filepath).expect("carai");
    let parsed_content = parser::parse_namespace(contents.as_bytes());
    match parsed_content {
        Ok((_, expressions)) => {
            for e in expressions {
                eval::eval(e, &mut env, &local_env);
            }
            let (_, main_call) = parser::parse("(main!)").expect("it is correctamundo");
            println!("{}", eval::eval(main_call, &mut env, &local_env));
        },
        Err(parser_error) => {
            println!("Fuck you, boah: {:?}", parser_error)// TODO: Parser errors could be turned into 'syntax errors'
        }
    }
}

fn main() {
    /* Construct built-in function table */
    let (mut env, local_env) = built_in::initial_env();

    let args: Vec<String> = std::env::args().collect();
    if &args[1] == "repl" {
        repl(&mut env, local_env)
    } else {
        load_main_namespace(&mut env, local_env, &args[1])
    }
}
