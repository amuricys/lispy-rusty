use types::{Expression, EvaluationError, EvalResult, SpecialForm, Atom, Environment, FunctionType, SinglyLinkedList};
use nom::lib::std::collections::{HashMap};
use util;

fn quote_expr(to_quote: Vec<Expression>) -> EvalResult<Expression> {
    if to_quote.len() == 1 {
        Ok(to_quote[0].clone())
    } else {
        Err(EvaluationError::WrongArity(1, to_quote.len() as i64))
    }

}

fn if_expr(args: Vec<Expression>, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<Expression> {
    let is_truthy: fn(Expression) -> bool = |expr|
        match expr {
            Expression::At(Atom::Bool(false)) => false,
            Expression::At(Atom::Nil) => false,
            _ => true
        };

    if args.len() == 3 || args.len() == 2 {
        let evaled_cond = eval_expression(args[0].clone(), env, local_vars)?;
        if is_truthy(evaled_cond) {
            eval_expression(args[1].clone(), env, local_vars)
        } else if args.len() == 3 {
            eval_expression(args[2].clone(), env, local_vars)
        } else {
            Ok(Expression::At(Atom::Nil))
        }
    } else {
        Err(EvaluationError::WrongArity(3, args.len() as i64))
    }
}

// TODO: Horrível
fn lambda (args: Vec<Expression>) -> EvalResult<Expression> {
    if args.len() == 2 {
        match args[0].clone() {
            Expression::Array(arr) => {
                let mut arr_iterator = arr.clone().into_iter();
                if arr_iterator.all(|x| match x { Expression::At(Atom::Symbol(_)) => true, _ => false}) {
                    Ok(Expression::Function(FunctionType::Lambda(arr.clone(), Box::from(args[1].clone()))))
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

fn def(args: Vec<Expression>, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<Expression> {
    if args.len() == 2 {
        match args[0].clone() {
            Expression::At(Atom::Symbol(sym)) => {
                match lookup_symbol(&sym, env, local_vars) {
                    Ok(Expression::SpecialForm(_)) => { Err(EvaluationError::ForbiddenDef("Cannot overwrite special form".parse().unwrap()))}
                    Ok(Expression::Function(FunctionType::BuiltIn(_))) => { Err(EvaluationError::ForbiddenDef("Cannot overwrite built in function".parse().unwrap()))}
                    _ => {
                        let value = eval_expression(args[1].clone(), env, local_vars)?;
                        env.top_level_vars.insert(sym, value.clone());
                        Ok(value)
                    }
                }
            }
            _ => Err(EvaluationError::UnknownBinding("Cannot def a non-symbol".parse().unwrap()))
        }
    } else {
        Err(EvaluationError::WrongArity(2, args.len() as i64))
    }
}

fn vector_of_tuples_to_hash_map(tuples: &Vec<(Expression, Expression)>, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<HashMap<String, Expression>> {
    let mut map_of_bound_names = HashMap::new();
    for (name, value) in tuples {
        match name {
            Expression::At(Atom::Symbol(sym)) => {
                let value_to_bind = eval_expression(value.clone(), env, local_vars)?;
                map_of_bound_names.insert(sym.clone(), value_to_bind);
            }
            _ => { return Err(EvaluationError::UnknownBinding("Illegal binding; only symbols".parse().unwrap())) }
        }
    }
    Ok(map_of_bound_names)
}

fn let_expr(args: Vec<Expression>, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<Expression> {
    if args.len() == 2 {
        match args[0].clone() {
            Expression::Array(vec) => {
                let vector_of_tuples = util::tuple_even_vector(vec)?;
                let map_of_bound_names = vector_of_tuples_to_hash_map(&vector_of_tuples, env, local_vars)?;
                let local_env_overwrites = SinglyLinkedList::Cons(map_of_bound_names, local_vars);
                eval_expression(args[1].clone(), env, &local_env_overwrites)
            }
            _ => { Err(EvaluationError::WrongType("Second argument of let binding must be array".parse().unwrap())) }
        }
    } else {
        Err(EvaluationError::WrongArity(2, args.len() as i64))
    }
}

fn call_function(called_fn: FunctionType, args_fn_is_receiving: Vec<Expression>, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<Expression> {
    let evaled_args = util::map_m(args_fn_is_receiving.into_iter().map(|x| eval_expression(x, env, local_vars)).collect());

    match evaled_args {
        Ok(correctly_evaled_args) => {
            match called_fn {
                FunctionType::BuiltIn(built_in) => built_in(correctly_evaled_args),
                FunctionType::Lambda(params_fn_takes, body) => {
                    let two_together = params_fn_takes.into_iter().zip(correctly_evaled_args).collect();
                    let map_of_bound_names = vector_of_tuples_to_hash_map(&two_together, env, local_vars)?;
                    let local_env_overwrites = SinglyLinkedList::Cons(map_of_bound_names, local_vars);
                    eval_expression(*body, env, &local_env_overwrites)
                },
            }
        }
        Err(err) => Err(err)
    }
}


fn call_special_form(special_form: SpecialForm, args: Vec<Expression>, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<Expression> {
    match special_form {
        SpecialForm::Quote => quote_expr(args),
        SpecialForm::If => if_expr(args, env, local_vars),
        SpecialForm::Fn => lambda(args),
        SpecialForm::Def => def(args, env, local_vars),
        SpecialForm::Let => let_expr(args, env, local_vars)
    }
}

fn lookup_vars<'a>(sym: &String, vars_layers: &'a SinglyLinkedList<HashMap<String, Expression>>) -> Option<&'a Expression> {
    match vars_layers {
        SinglyLinkedList::Cons(vars, tail) => {
            if let Option::Some(val) = vars.get(sym) {
                Option::Some(val)
            } else {
                lookup_vars(sym, tail)
            }
        }
        SinglyLinkedList::Nil => { Option::None }
    }
}

fn lookup_symbol(sym: &String, env: &Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>> ) -> EvalResult<Expression> {
    if let Some(special_form_type) = env.special_forms.get(sym) {
        Ok(Expression::SpecialForm(special_form_type.clone()))
    } else if let Some(built_in) = env.built_in_fns.get(sym) {
        Ok(Expression::Function(FunctionType::BuiltIn(*built_in)))
    } else if let Some(expr) = lookup_vars(sym, local_vars) {
        Ok(expr.clone())
    } else if let Some(expr) = env.top_level_vars.get(sym) {
        Ok(expr.clone())
    } else {
        Err(EvaluationError::SymbolNotFound(sym.parse().unwrap()))
    }
}

fn eval_expression(expr: Expression, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> EvalResult<Expression> {
    match expr {
        /* TODO:  This match should be further broken up. Each collection should have its own logic. */
        Expression::At(Atom::Symbol(sym)) => lookup_symbol(&sym, env, local_vars),
        Expression::At(_) => Ok(expr),
        Expression::Function(_) => Ok(expr),
        Expression::SpecialForm(_) => Err(EvaluationError::SpecialFormOutOfContext), // TODO: Impossible due to lookup
        Expression::Array(_) => Ok(expr), // TODO: Has to eval elements
        Expression::Map(_) => Ok(expr),   // TODO: Has to eval elements. What does eval'ing a map return? A HashMap that is only internal?
        Expression::List(op, args) => {
            let caller = eval_expression(*op, env, local_vars)?;
            match caller {
                Expression::SpecialForm(special_form_type) => call_special_form(special_form_type.clone(), args, env, local_vars),
                Expression::Function(fn_type) => call_function(fn_type, args, env, local_vars),
                _ => Err(EvaluationError::NotSpecialFormOrFunction)
            }
        }
    }
}

pub fn eval(expr: Expression, env: &mut Environment, local_vars: &SinglyLinkedList<HashMap<String, Expression>>) -> String {
    let evaled_expr = eval_expression(expr, env, local_vars);
    format!("{:?}", evaled_expr)
}
