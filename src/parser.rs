use nom::character::{is_space, is_digit};

#[derive(Debug)]
struct Expression {
    operator: char,
    operand1: i32,
    operand2: i32
}

/* Defining the polish notation parser
 * Eg:
 *      + 1 2 = 1 + 2
 */
named!(
    operators<Expression>,
    do_parse!(
        operator: one_of!("+-/*") >>
        take_while1!(is_space) >>
        operand1: take_while1!(is_digit) >>
        take_while1!(is_space) >>
        operand2: take_while1!(is_digit) >>
        (Expression{
            operator: operator,
            operand1: from_u8_array_to_i32(operand1),
            operand2: from_u8_array_to_i32(operand2)
        })
    )
);

fn from_u8_array_to_i32(input: &[u8]) -> i32{
    std::str::from_utf8(input)
            . expect("Error byte array -> string")
            . parse()
            . expect("Error string -> i32")
}

pub fn parse(input : &str) -> String {
    let expression = operators(input.as_bytes());
    println!("{:?}", expression);
    // FIXME: no explaination needed here unless you have Deco's IQ
    String::from("aaa")
}
