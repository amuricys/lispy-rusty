use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::{is_space, is_digit},
    character::complete::{char, one_of},
    combinator::{cut, map, opt, value},
    Err, IResult,
};

#[derive(Debug)]
enum Expression2 {
    Num(i32),
    Expr(char, Box<Expression2>, Box<Expression2>)
}

/* WIP: Structure of recursive parser seems correct, we're fighting the nom DSL syntax now.
   An expression is either a simple literal (first branch of the alt! below) , or an open-parentheses
   expression with an operator in front. It actually doesn't have to be an operator, could be yet another
   open-parentheses expression. Lisp expressions can simply be lists of n other expressions,
   meaning the following is valid:

      (((((function-that-returns-another-function arg1 arg2) arg3) arg4) arg5) arg6)

   But we'll pretend the first element has to be an operator for now, and that the operands can be recursive. */

fn parse_num2(i: &[u8]) -> IResult<&[u8], Expression2, (&[u8], nom::error::ErrorKind)> {
    let first_spaces: IResult<&[u8], &[u8], (&[u8], nom::error::ErrorKind)> = take_while(is_space)(i);
    match first_spaces {
        Ok((rest, _)) => {
            let digits: IResult<&[u8], &[u8], (&[u8], nom::error::ErrorKind)> = take_while1(is_digit)(rest);
            match digits {
                Ok((rest, num)) => {
                    let end_spaces:  IResult<&[u8], &[u8], (&[u8], nom::error::ErrorKind)> = take_while(is_space)(rest);
                    match end_spaces {
                        Ok((rest, _)) => {
                            Ok((rest, Expression2::Num(from_u8_array_to_i32(num))))
                        }
                        Err(_) => {
                            panic!("Impossivel kk")
                        }
                    }
                }
                Err(_) =>
                    panic!("Preguiça de fazer essa porra desse pattern matching funcionar")
            }
        }
        Err(_) => panic!("Impossivel he!")
    }
}

fn parse_expression2(i: &[u8]) -> IResult<&[u8], Expression2, (&[u8], nom::error::ErrorKind)> {
    let first_spaces: IResult<&[u8], &[u8], (&[u8], nom::error::ErrorKind)> = take_while(is_space)(i);
    // ai aqui falta a porra do operador kk mas porra da pra ver que ta indo bem a porra
    match first_spaces {
        Ok((rest, _)) => {
            let operand1: IResult<&[u8], Expression2, (&[u8], nom::error::ErrorKind)> = parse_expression_top_level(rest);
            match operand1 {
                Ok((rest, exprOp1)) => {
                    let operand2: IResult<&[u8], Expression2, (&[u8], nom::error::ErrorKind)> = parse_expression_top_level(rest);
                    match operand2 {
                        Ok((rest, _)) => {
                            Ok((rest, Expression2::Num(from_u8_array_to_i32(num))))
                        }
                        Err(_) => {
                            panic!("Impossivel kk")
                        }
                    }
                }
                Err(_) =>
                    panic!("Preguiça de fazer essa porra desse pattern matching funcionar")
            }
        }
        Err(_) => panic!("Impossivel he!")
    }
}

fn parse_expression_top_level(i: &[u8]) -> IResult<&[u8], Expression2, (&[u8], nom::error::ErrorKind)> {
    alt ((parse_expression, parse_num2))(i)
}

named!(
    parse_num<Expression2>,
    do_parse!(
        take_while!(is_space) >>
        num: take_while1!(is_digit) >>
        take_while!(is_space) >>
        (Expression2::Num(
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
    let expression = parse_expression_top_level(input.as_bytes());
    println!("{:?}", expression);
    // FIXME: no explaination needed here unless you have Deco's IQ
    String::from("aaa")
}
