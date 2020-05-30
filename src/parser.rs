use nom::character::{is_space, is_digit};

#[derive(Debug)]
struct Expression {
    operator: char,
    operand1: i32,
    operand2: i32
}

enum Expression2 {
    Num(i32),
    Expr(char, Box<Expression2>, Box<Expression2>)
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
/* WIP: Structure of recursive parser seems correct, we're fighting the nom DSL syntax now.
   An expression is either a simple literal (first branch of the alt! below) , or an open-parentheses
   expression with an operator in front. It actually doesn't have to be an operator, could be yet another
   open-parentheses expression. Lisp expressions can simply be lists of n other expressions,
   meaning the following is valid:

      (((((function-that-returns-another-function arg1 arg2) arg3) arg4) arg5) arg6)

   But we'll pretend the first element has to be an operator for now, and that the operands can be recursive. */
named!(
    parse_expression_top_level<Expression2>,
    alt! (
        parse_num |
        parse_expression
    )
);

named!(
    parse_num<Expression2>,
    do_parse!(
        take_while!(is_space) >>
        num: take_while1!(is_digit) >>
        take_while!(is_space) >>
        (Num(
            from_u8_array_to_i32(num)
        ))
    )
);

named!(
    parse_expression<Expression2>,
    delimited!(
        char!('('),
        do_parse!(
            operator: one_of!("+-/*") >>
            take_while1!(is_space) >>
            operand1: parse_expression_top_level() >>
            take_while1!(is_space) >>
            operand2: parse_expression_top_level() >>
            take_while1!(is_space) >>
            (Expr(
                operator,
                operand1,
                operand2
             ))),
        char!(')'))

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
