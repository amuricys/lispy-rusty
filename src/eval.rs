use std::collections::HashMap;
use types::{Expression, EvaluationError, EvalResult, SpecialForm, Atom, Environment, FunctionType};
use types::Expression::{List, At};

fn quote_expr(to_quote: Vec<Expression>) -> EvalResult<Expression> {
    if to_quote.len() == 1 {
        Ok(to_quote[0].clone())
    } else {
        Err(EvaluationError::WrongArity(1, to_quote.len() as i64))
    }

}

fn if_expr(args: Vec<Expression>, env: &Environment) -> EvalResult<Expression> {
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

// TODO 4: Map_m específico nojento
fn map_m(maybes: Vec<EvalResult<Expression>>) -> Result<Vec<Expression>, EvalResult<Expression>> {
    let mut to_return = Vec::new();
    for eval_res in maybes {
        match eval_res {
            Err(_) => return Err(eval_res),
            Ok(expr) => to_return.push(expr)
        }
    }
    Ok(to_return)
}

// TODO: Horrível
fn lambda (args: Vec<Expression>) -> EvalResult<Expression> {
    if args.len() == 2 {
        match args[0].clone() {
            Expression::Array(arr) => {
                let mut arr_iterator = arr.clone().into_iter();
                if arr_iterator.all(|x| match x { Expression::At(Atom::Symbol(_)) => true, _ => false}) {
                    let arr_atoms = arr.into_iter().map(|x |
                        match x {
                            Expression::At(Atom::Symbol(s)) => Atom::Symbol(s),
                            _ => panic!("acho que impossivel")
                        });
                    Ok(Expression::Function(FunctionType::Lambda(arr_atoms.collect(), Box::from(args[1].clone()))))
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

// TODO 2: Horrível. Fuck for loops
fn substitute(params_fn_takes: Vec<Atom>, args_fn_is_receiving: Vec<Expression>, env: &Environment) -> EvalResult<Environment> {
    let mut new_environment = env.clone(); // TODO: Don't clone whole environment for each function call
    let two_together = params_fn_takes.into_iter().zip(args_fn_is_receiving);
    // TODO: disgusting side-effectful map, should be a mapM_
    // let res = two_together.map(|(param,arg)| {
    //     match param {
    //         Atom::Symbol(sym) => Ok(new_environment.vars.insert(sym, arg)),
    //         _ => Err(EvaluationError::UnknownBinding("can only bind to symbols".parse().unwrap()))
    //     }
    // });
    for (param, arg) in two_together {
        match param {
            Atom::Symbol(sym) => new_environment.vars.insert(sym, arg),
            _ => return Err(EvaluationError::UnknownBinding("can only bind to symbols".parse().unwrap()))
        };
    }
    Ok(new_environment)
}

// TODO 3: Usar um map_m sério
fn call_function(called_fn: FunctionType, args_fn_is_receiving: Vec<Expression>, env: &Environment) -> EvalResult<Expression> {
    let evaled_args = map_m(args_fn_is_receiving.into_iter().map(|x| eval_expression(x, env)).collect());

    match evaled_args {
        Ok(correctly_evaled_args) => {
            match called_fn {
                FunctionType::Lambda(params_fn_takes, body) => {
                    let local_env = substitute(params_fn_takes, correctly_evaled_args, env)?;
                    eval_expression(*body, &local_env)
                },
                FunctionType::BuiltIn(built_in) => built_in(correctly_evaled_args),
            }
        }
        Err(err) => err
    }
}


fn call_special_form(special_form: SpecialForm, args: Vec<Expression>, env: &Environment) -> EvalResult<Expression> {
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

fn lookup_symbol(sym: String, env: &Environment) -> EvalResult<Expression> {
    if let Some(special_form_type) = env.special_forms.get(&sym) {
        Ok(Expression::SpecialForm(special_form_type.clone()))
    } else if let Some(built_in) = env.built_in_fns.get(&sym) {
        Ok(Expression::Function(FunctionType::BuiltIn(*built_in)))
    } else if let Some(expr) = env.vars.get(&sym) {
        Ok(expr.clone())
    } else {
        Err(EvaluationError::SymbolNotFound(sym))
    }
}

fn eval_expression(expr: Expression, env: &Environment) -> EvalResult<Expression> {
    match expr {
        Expression::At(Atom::Symbol(sym)) => lookup_symbol(sym, env),
        Expression::At(_) => Ok(expr),
        Expression::Function(_) => Ok(expr),
        Expression::SpecialForm(_) => Err(EvaluationError::SpecialFormOutOfContext), // TODO: Impossible due to lookup
        Expression::Array(_) => Ok(expr), // TODO: Has to eval elements
        Expression::List(op, args) => {
            let caller = eval_expression(*op, env)?;
            match caller {
                Expression::SpecialForm(special_form_type) => call_special_form(special_form_type.clone(), args, &env),
                Expression::Function(fn_type) => call_function(fn_type, args, &env),
                _ => Err(EvaluationError::NotSpecialFormOrFunction)
            }
        }
    }
}

pub fn eval(expr: Expression, env: &Environment) -> String {
    let evaled_expr = eval_expression(expr, env);
    format!("{:?}", evaled_expr)
}
