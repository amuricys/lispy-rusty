use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1},
    sequence::delimited,
    character::{is_space, is_digit},
    character::complete::{char, one_of},
    combinator::{cut, map, opt, value},
    Err, IResult,
};

#[derive(Debug)]
pub enum Expression {
    Num(i32),
    Expr(char, Box<Expression>, Box<Expression>),
}

/* WIP: Structure of recursive parser seems correct, we're fighting the nom DSL syntax now.
   An expression is either a simple literal (first branch of the alt below) , or an open-parentheses
   expression with an operator in front. It actually doesn't have to be an operator, could be yet another
   open-parentheses expression. Lisp expressions can simply be lists of n other expressions,
   meaning the following is valid:

      (((((function-that-returns-another-function arg1 arg2) arg3) arg4) arg5) arg6)

   But we'll pretend the first element has to be an operator for now, and that the operands can be recursive. */

fn parse_expression_top_level(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    alt((parse_num, parse_expression))(i)
}

fn parse_num(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _first_spaces) = take_while(is_space)(i)?;
    let (rest, digit) = take_while1(is_digit)(rest)?;
    let (rest, _end_spaces) = take_while(is_space)(rest)?;

    Ok((rest, Expression::Num(from_u8_array_to_i32(digit))))
}

fn parse_expression(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _paren1) = char('(')(i)?;
    let (rest, _first_spaces) = take_while(is_space)(rest)?;
    let (rest, op) = one_of("+-/*")(rest)?;
    let (rest, _spaces_between_operator_and_first_operand) = take_while1(is_space)(rest)?;
    let (rest, expr_op1) = parse_expression_top_level(rest)?;
    let (rest, expr_op2) = parse_expression_top_level(rest)?;
    let (rest, _paren2) = char(')')(rest)?;

    Ok((rest, Expression::Expr(op, Box::new(expr_op1), Box::new(expr_op2))))
}

fn from_u8_array_to_i32(input: &[u8]) -> i32 {
    std::str::from_utf8(input)
        .expect("Error byte array -> string")
        .parse()
        .expect("Error string -> i32")
}

pub fn parse(input: &str) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let expression = parse_expression_top_level(input.as_bytes());
    expression
}

pub fn eval (expr: Expression) -> String {
     format!("{:?}", expr)
}
