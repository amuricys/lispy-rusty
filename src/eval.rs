use parser::{Expression, Atom};
use std::collections::HashMap;

#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    FunctionNotFound,
    NotAFunction,
    WrongType(String),
    WrongArity(i64, i64) // (Expected, Received)
}


/* TODO: Discover how to parametrize the Ok return value
   E.g.:  When evaluation of summation goes OK, the result is not
   just any Expression, it is in fact an Expression that contains an integer.
   Any value of that return Expression is going to successfully pattern match on
   the pattern
   Expression::At(Atom::Int(val))

   How do we get this guarantee on the type level? */
pub type EvalResult = Result<Expression, EvaluationError>;


fn map_m(maybes: Vec<EvalResult>) -> Result<Vec<Expression>, EvalResult> {
    let mut to_return = Vec::new();
    for eval_res in maybes {
        match eval_res {
            Err(_) =>{ return Err(eval_res) }
            Ok(expr) => { to_return.push(expr) }
        }
    }
    Ok(to_return)
}

fn eval_expression(expr: Expression, vars: &HashMap<String, fn(Vec<Expression>) -> EvalResult>) -> EvalResult {
    match expr {
        Expression::At(_) => { Ok(expr) }
        Expression::Expr(op, args) => {
            let evaled_fn_symbol = eval_expression(*op, vars)?;
            let evaled_args = args.into_iter().map(|x| eval_expression(x, vars));
            let try_correct_evaled_args = map_m(evaled_args.collect());
            match try_correct_evaled_args {
                Err(err) => { err }
                Ok(evaled_args) => {
                    match evaled_fn_symbol {
                        Expression::At(Atom::Symbol(sym)) => {
                            match vars.get(&sym) {
                                None => { Err(EvaluationError::FunctionNotFound) }
                                Some(rust_fn) => { rust_fn(evaled_args) }
                            }
                        }
                        _ => {Err(EvaluationError::NotAFunction)}
                    }
                }
            }
        }
    }
}

pub fn eval(expr: Expression, vars: &HashMap<String, fn(Vec<Expression>) -> EvalResult>) -> String {
    let evaled_expr = eval_expression(expr, vars);
    format!("{:?}", evaled_expr)
}
