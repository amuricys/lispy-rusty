use std::collections::HashMap;
use types::{Expression,EvaluationError,ResolvedSymbol,EvalResult,SpecialForm,Atom};
use types::Expression::{Expr, At};

fn quote_expr(to_quote: Vec<Expression>) -> EvalResult {
    if to_quote.len() == 1 {
        Ok(to_quote[0].clone())
    } else {
        Err(EvaluationError::WrongArity(1, to_quote.len() as i64))
    }

}

fn if_expr(args: Vec<Expression>, vars: &HashMap<String, ResolvedSymbol>) -> EvalResult {
    let is_truthy: fn(Expression) -> bool = |expr|
        match expr {
            Expression::At(Atom::Bool(false)) => false,
            Expression::At(Atom::Nil) => false,
            _ => true
        };

    if args.len() == 3 || args.len() == 2 {
        let evaled_cond = eval_expression(args[0].clone(), vars)?;
        if is_truthy(evaled_cond) {
            eval_expression(args[1].clone(), vars)
        } else if args.len() == 3 {
            eval_expression(args[2].clone(), vars)
        } else {
            Ok(Expression::At(Atom::Nil))
        }
    } else {
        Err(EvaluationError::WrongArity(3, args.len() as i64))
    }
}

fn map_m(maybes: Vec<EvalResult>) -> Result<Vec<Expression>, EvalResult> {
    let mut to_return = Vec::new();
    for eval_res in maybes {
        match eval_res {
            Err(_) => return Err(eval_res),
            Ok(expr) => to_return.push(expr)
        }
    }
    Ok(to_return)
}

fn call_function(called_functon: Expression, args_fn_is_receiving: Vec<Expression>, vars: &HashMap<String, ResolvedSymbol>) -> EvalResult {
    match called_functon {
        Expression::Function(params_fn_takes, body) => Ok(Expression::At(Atom::Int(1))),
        _ => Err(EvaluationError::NotAFunction)
    }
}


fn call_special_form(special_form: SpecialForm, args: Vec<Expression>, vars: &HashMap<String, ResolvedSymbol>) -> EvalResult {
    match special_form {
        SpecialForm::Quote => quote_expr(args),
        SpecialForm::If => if_expr(args, vars),
        _ => { panic! ("Como vc achou um special form sem que ele exista? Lixo")}
        /* TODO: Implement other special forms
        SpecialForm::Fn(args, body) => { Ok(*my_boxed_expr) }
        SpecialForm::Def(name, value) => { Ok(*my_boxed_expr) }
        */
    }
}


fn eval_expression(expr: Expression, vars: &HashMap<String, ResolvedSymbol>) -> EvalResult {
    match expr {
        Expression::At(_) => Ok(expr),
        Expression::Function(_,_) => Ok(expr),
        Expression::Expr(op, args) => {
            let caller = eval_expression(*op, vars)?;
            match caller {
                Expression::At(Atom::Symbol(sym)) => {
                    let resolved_symbol = vars.get(&sym);
                    match resolved_symbol {
                        Some(ResolvedSymbol::SpecialF(special_form_type)) => call_special_form(special_form_type.clone(), args, vars),
                        Some(ResolvedSymbol::Value(Expression::Function(params, body))) => call_function(Expression::Function(params.clone(), body.clone()), args, vars),
                        _ => Err(EvaluationError::SymbolNotFound)
                    }
                }
                Expression::Function(params, body) => call_function(Expression::Function(params.clone(), body.clone()), args, vars),
                _ => Err(EvaluationError::NotAFunction)
            }
        }
    }
}

pub fn eval(expr: Expression, vars: &HashMap<String, ResolvedSymbol>) -> String {
    let evaled_expr = eval_expression(expr, vars);
    format!("{:?}", evaled_expr)
}
