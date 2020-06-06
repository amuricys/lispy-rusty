use nom::{branch::alt,
          bytes::complete::{take_while, take_while1, take_till, tag, is_not},
          sequence::delimited, character::{is_space, is_digit},
          character::complete::{char, one_of},
          combinator::{cut, map, opt, value},
          Err, IResult, AsChar};

#[derive(Debug)]
pub enum Atom {
    Int(i64),
    Char(char),
    Bool(bool),
}

#[derive(Debug)]
pub enum Expression {
    At(Atom),
    Expr(Box<Expression>, Vec<Expression>),
}

/* WIP: Structure of recursive parser seems correct, we're fighting the nom DSL syntax now.
   An expression is either a simple literal (first branch of the alt below) , or an open-parentheses
   expression with an operator in front. It actually doesn't have to be an operator, could be yet another
   open-parentheses expression. Lisp expressions can simply be lists of n other expressions,
   meaning the following is valid:

      (((((function-that-returns-another-function arg1 arg2) arg3) arg4) arg5) arg6)

   But we'll pretend the first element has to be an operator for now, and that the operands can be recursive. */

fn parse_expression_top_level(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    alt((parse_atom, parse_expression))(i)
}

fn parse_num(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _first_spaces) = take_while(is_space)(i)?;
    let (rest, digit) = take_while1(is_digit)(rest)?;
    let (rest, _end_spaces) = take_while(is_space)(rest)?;

    Ok((rest, Expression::At(Atom::Int(from_u8_array_to_i64(digit)))))
}

fn parse_char(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _first_spaces) = take_while(is_space)(i)?;
    let (rest, _escape) = char('\\')(rest)?;
    let (rest, char_string) =
        alt((is_not("\\"),
             alt((tag("\\n"),
                  tag("\\t"),
                  tag("\\r")))))
            (rest)?;
    let (rest, _end_spaces) = take_while1(is_space)(rest)?; // something like \abcdef is not valid

    Ok((rest, Expression::At(Atom::Char(char_string[0].as_char()))))
}

fn parse_bool(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _first_spaces) = take_while(is_space)(i)?;
    let (rest, boolean) = alt((tag("true"), tag("false")))(rest)?;
    let (rest, _end_spaces) = take_while(is_space)(rest)?;

    Ok((rest, Expression::At(Atom::Bool(from_u8_array_to_bool(boolean)))))
}

fn parse_atom(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    alt((parse_num, parse_char, parse_bool))(i)
}

fn parse_expression(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _paren1) = char('(')(i)?;
    let (rest, _first_spaces) = take_while(is_space)(rest)?;
    let (rest, op_expression) = parse_expression_top_level(rest)?;
    let (rest, _spaces_between_operator_and_first_operand) = take_while(is_space)(rest)?;
    let (rest, expr_op1) = parse_expression_top_level(rest)?;
    let (rest, expr_op2) = parse_expression_top_level(rest)?;
    let (rest, _paren2) = char(')')(rest)?;

    let mut args = Vec::new();
    args.push(expr_op1);
    args.push(expr_op2);

    Ok((rest, Expression::Expr(Box::new(op_expression), args)))
}

fn from_u8_array_to_i64(input: &[u8]) -> i64 {
    std::str::from_utf8(input)
        .expect("Error byte array -> string")
        .parse()
        .expect("Error string -> i64")
}

fn from_u8_array_to_bool(input: &[u8]) -> bool {
    std::str::from_utf8(input)
        .expect("Error byte array -> string")
        .parse()
        .expect("Error string -> bool")
}

pub fn parse(input: &str) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let expression = parse_expression_top_level(input.as_bytes());
    println!("{:?}", expression);
    expression
}

pub fn eval(expr: Expression) -> String {
    format!("{:?}", expr)
}
