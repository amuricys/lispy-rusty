use std::collections::HashMap;
use types::{Expression, EvaluationError, EvalResult, SpecialForm, Atom, Environment};
use types::Expression::{Expr, At};

fn quote_expr(to_quote: Vec<Expression>) -> EvalResult {
    if to_quote.len() == 1 {
        Ok(to_quote[0].clone())
    } else {
        Err(EvaluationError::WrongArity(1, to_quote.len() as i64))
    }

}

fn if_expr(args: Vec<Expression>, env: &Environment) -> EvalResult {
    let is_truthy: fn(Expression) -> bool = |expr|
        match expr {
            Expression::At(Atom::Bool(false)) => false,
            Expression::At(Atom::Nil) => false,
            _ => true
        };

    if args.len() == 3 || args.len() == 2 {
        let evaled_cond = eval_expression(args[0].clone(), env)?;
        if is_truthy(evaled_cond) {
            eval_expression(args[1].clone(), env)
        } else if args.len() == 3 {
            eval_expression(args[2].clone(), env)
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

fn lambda (args: Vec<Expression>) -> EvalResult {
    if args.len() == 2 {
        match args[0].clone() {
            Expression::Array(arr) => {
                let mut arr_iterator = arr.into_iter();
                if arr_iterator.all(|x| match x { Expression::At(Atom::Symbol(_)) => true, _ => false}) {
                    let arr_atoms = arr_iterator.map(|x |
                        match x {
                            Expression::At(Atom::Symbol(s)) => Atom::Symbol(s),
                            _ => panic!("acho que impossivel")
                        });
                    Ok(Expression::Lambda(arr_atoms.collect(), Box::from(args[1].clone())))
                } else {
                    Err(EvaluationError::WrongType("not all args were symbols".parse().unwrap()))
                }
            }
            _ => Err(EvaluationError::WrongType("arguments to fn must be array".parse().unwrap()))
        }
    } else {
        Err(EvaluationError::WrongArity(2, args.len() as i64))
    }
}

fn call_function(called_functon: Expression, args_fn_is_receiving: Vec<Expression>, env: &Environment) -> EvalResult {
    match called_functon {
        Expression::Lambda(params_fn_takes, body) => {
            Ok(Expression::At(Atom::Int(1)))
        },
        _ => Err(EvaluationError::NotSpecialFormOrFunction) // TODO Should be impossible. How to express with types?
    }
}


fn call_special_form(special_form: SpecialForm, args: Vec<Expression>, env: &Environment) -> EvalResult {
    match special_form {
        SpecialForm::Quote => quote_expr(args),
        SpecialForm::If => if_expr(args, env),
        SpecialForm::Fn => lambda(args),
        _ => { panic! ("Como vc achou um special form sem que ele exista? Lixo")}
        /* TODO: Implement other special forms
        SpecialForm::Def(name, value) => { Ok(*my_boxed_expr) }
        */
    }
}

fn lookup_symbol(sym: String, env: &Environment) -> EvalResult {
    if let Some(special_form_type) = env.special_forms.get(&sym) {
        Ok(Expression::SpecialForm(special_form_type.clone()))
    } else if let Some(built_in) = env.built_in_fns.get(&sym) {
        Ok(Expression::BuiltIn(*built_in))
    } else if let Some(expr) = env.vars.get(&sym) {
        Ok(expr.clone())
    } else {
        Err(EvaluationError::SymbolNotFound)
    }
}

fn eval_expression(expr: Expression, env: &Environment) -> EvalResult {
    match expr {
        Expression::At(Atom::Symbol(sym)) => lookup_symbol(sym, env),
        Expression::At(_) => Ok(expr),
        Expression::BuiltIn(_) => Ok(expr),
        Expression::Lambda(_, _) => Ok(expr),
        Expression::Array(_) => Ok(expr),
        Expression::SpecialForm(_) => Err(EvaluationError::SpecialFormOutOfContext),
        Expression::Expr(op, args) => {
            let caller = eval_expression(*op, env)?;
            match caller {
                Expression::SpecialForm(special_form_type) => call_special_form(special_form_type.clone(), args, &env),
                Expression::BuiltIn(built_in) => built_in(args), // TODO: Built-ins have to evaluate their arguments beforehand too
                Expression::Lambda(params, body) => call_function(Expression::Lambda(params.clone(), body.clone()), args, &env),
                _ => Err(EvaluationError::NotSpecialFormOrFunction)
            }
        }
    }
}

pub fn eval(expr: Expression, env: &Environment) -> String {
    let evaled_expr = eval_expression(expr, env);
    format!("{:?}", evaled_expr)
}
