use parser::{Expression, Atom};
use std::collections::HashMap;

#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    FunctionNotFound,
    NotAFunction,
    WrongType(String),
    WrongArity(i64, i64), // (Expected, Received)
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
            Err(_) => { return Err(eval_res); }
            Ok(expr) => { to_return.push(expr) }
        }
    }
    Ok(to_return)
}

#[derive (Debug, Clone)]
pub enum SpecialForm {
    /*  (QExpression) */
    Quote(Box<Expression>),
    /*  (Cond, If Branch, Else Branch) */
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    /*  (ArgList, Body) */
    Fn(Box<Expression>, Box<Expression>),
    /*  (Name, Value) */
    Def(Box<Expression>, Box<Expression>),
}


#[derive (Debug, Clone)]
pub enum ResolvedSymbol {
    Value(Expression),
    SpecialF(SpecialForm),
}

fn call_function(args: Vec<Expression>) -> EvalResult {
    let evaled_args = args.into_iter().map(|x| eval_expression(x, functions_available));
    let try_correct_evaled_args = map_m(evaled_args.collect());
    match try_correct_evaled_args {
        Err(err) => { err }
        Ok(evaled_args) => {
            match checked_first_symbol {
                Expression::At(Atom::Symbol(sym)) => {
                    match functions_available.get(&sym) {
                        None => { Err(EvaluationError::FunctionNotFound) }
                        Some(rust_fn) => { rust_fn(evaled_args) }
                    }
                }
                _ => { Err(EvaluationError::NotAFunction) }
            }
        }
    }
}

fn call_special_form(special_form: SpecialForm) -> EvalResult {
    match special_form {
        SpecialForm::Quote(my_boxed_expr) => { Ok(*my_boxed_expr) }
        _ => { panic! ("Como vc achou um special form sem que ele exista? Lixo")}
        /* TODO: Implement other special forms
        SpecialForm::If(cond_e, true_e, false_e) => { Ok(*my_boxed_expr) }
        SpecialForm::Fn(args, body) => { Ok(*my_boxed_expr) }
        SpecialForm::Def(name, value) => { Ok(*my_boxed_expr) }
        */
    }
}

fn eval_expression(expr: Expression, vars: &HashMap<String, ResolvedSymbol>) -> EvalResult {
    match expr {
        Expression::At(_) => { Ok(expr) }
        Expression::Expr(op, args) => {
            let checked_first_symbol = eval_expression(*op, vars)?;
            match checked_first_symbol {
                Expression::At(Atom::Symbol(sym)) => {
                    let resolved_symbol = vars.get(&sym);
                    match resolved_symbol {
                        Some(ResolvedSymbol::SpecialF(minha_pica)) => {
                            call_special_form(minha_pica.clone())
                        }
                        Some(ResolvedSymbol::Value(minha_pica)) => {
                            // Chama funcao
                            call_function("Sei la com o que vtnc")
                        }
                    }
                }
                _ => panic!("Ô caralho, mas que porra é essa??!?!")
            }
        }
    }
}

pub fn eval(expr: Expression, vars: &HashMap<String, ResolvedSymbol>) -> String {
    let evaled_expr = eval_expression(expr, vars);
    format!("{:?}", evaled_expr)
}
