use std::io::{self, Write};

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

        /* Print user input line */
        println!("{}", x);
    }
}
