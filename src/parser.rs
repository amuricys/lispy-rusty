use nom::{branch::alt,
          bytes::complete::{take_while, take_while1, tag, take},
          character::{is_space, is_digit},
          character::complete::{char},
          IResult, AsChar};
use nom::character::is_alphanumeric;

#[derive(Debug)]
pub enum Atom {
    Int(i64),
    Char(char),
    Bool(bool),
    Symbol(String),
}

#[derive(Debug)]
pub enum Expression {
    At(Atom),
    Expr(Box<Expression>, Vec<Expression>),
}

fn is_space_lisp(c: u8) -> bool {
    is_space(c) || c.as_char() == ','
}

fn is_valid_symbol_char(c: u8) -> bool {
    is_alphanumeric(c) || "*/+-_!?><".chars().any(|x| x == c.as_char())
}

fn parse_expression_top_level(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    alt((parse_atom, parse_expression))(i)
}

fn parse_num(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _spaces) = take_while(is_space_lisp)(i)?;
    let (rest, digit) = take_while1(is_digit)(rest)?;

    Ok((rest, Expression::At(Atom::Int(from_u8_array_to_i64(digit)))))
}

fn parse_char(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _spaces) = take_while(is_space_lisp)(i)?;
    let (rest, _escape) = char('\\')(rest)?;
    let (rest, char_string) =
        alt((tag("\\n"),
             tag("\\t"),
             tag("\\r"),
             tag("\\"),
             take(1 as usize)))(rest)?;

    // TODO: check if char string has size 2 to correctly parse \n, \t etc
    Ok((rest, Expression::At(Atom::Char(char_string[0].as_char()))))
}

fn parse_symbol(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _spaces) = take_while(is_space_lisp)(i)?;
    let (rest, symbol) = take_while1(is_valid_symbol_char)(rest)?;

    let wat = String::from_utf8(Vec::from(symbol)).expect("Me dê uma porra de uma string");

    Ok((rest, Expression::At(Atom::Symbol(wat))))
}

fn parse_bool(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _spaces) = take_while(is_space_lisp)(i)?;
    let (rest, boolean) = alt((tag("true"), tag("false")))(rest)?;

    Ok((rest, Expression::At(Atom::Bool(from_u8_array_to_bool(boolean)))))
}

fn parse_atom(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    alt((parse_num, parse_char, parse_bool, parse_symbol))(i)
}

fn parse_expression(i: &[u8]) -> IResult<&[u8], Expression, (&[u8], nom::error::ErrorKind)> {
    let (rest, _first_spaces) = take_while(is_space_lisp)(i)?;
    let (rest, _paren1) = char('(')(rest)?;
    let (rest, op_expression) = parse_expression_top_level(rest)?;

    let mut args = Vec::new();

    let mut new_rest = rest;
    let mut should_continue = true;
    while should_continue {
        let could_parse = parse_expression_top_level(new_rest);
        match could_parse {
            Ok((other_new_rest, expr)) => {
                args.push(expr);
                new_rest = other_new_rest;
            }
            Err(_) => {
                should_continue = false;
            }
        }
    }
    let (new_rest, _end_spaces) = take_while(is_space_lisp)(new_rest)?;
    let (newest_rest, _paren2) = char(')')(new_rest)?;

    Ok((newest_rest, Expression::Expr(Box::new(op_expression), args)))
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
