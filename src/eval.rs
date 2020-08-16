use types::{Expression, EvaluationError, EvalResult, SpecialForm, Atom, Environment, FunctionType, SinglyLinkedList, MapType};
use nom::lib::std::collections::{HashMap, VecDeque};
use util;

pub type LocalVars<'a> = SinglyLinkedList<'a, HashMap<String, Expression>>;

fn quote_expr(to_quote: VecDeque<Expression>) -> EvalResult<Expression> {
    if to_quote.len() == 1 {
        Ok(to_quote[0].clone())
    } else {
        Err(EvaluationError::WrongArity(1, to_quote.len() as i64))
    }
}

fn if_expr(args: VecDeque<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
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
fn lambda (args: VecDeque<Expression>) -> EvalResult<Expression> {
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

fn def(args: VecDeque<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
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

fn vector_of_tuples_to_hash_map_symbol(tuples: &Vec<(Expression, Expression)>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<HashMap<String, Expression>> {
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

fn vector_of_tuples_to_hash_map_atom(tuples: &Vec<(Expression, Expression)>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<HashMap<Atom, Expression>> {
    let mut map_of_bound_names = HashMap::new();
    for (name, value) in tuples {
        match name {
            Expression::At(atom) => {
                let value_to_bind = eval_expression(value.clone(), env, local_vars)?;
                map_of_bound_names.insert(atom.clone(), value_to_bind);
            }
            _ => { return Err(EvaluationError::UnknownBinding("Illegal binding; only symbols".parse().unwrap())) }
        }
    }
    Ok(map_of_bound_names)
}

fn let_expr(args: VecDeque<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    if args.len() == 2 {
        match args[0].clone() {
            Expression::Array(vec) => {
                let vector_of_tuples = util::tuple_even_vector(vec)?;
                /* TODO: This binds the let binding as a chunk! Meaning (let [x 10 y (+ 20)]) doesnt work as x is not visible to the second binding*/
                let map_of_bound_names = vector_of_tuples_to_hash_map_symbol(&vector_of_tuples, env, local_vars)?;
                let local_env_overwrites = SinglyLinkedList::Cons(map_of_bound_names, local_vars);
                eval_expression(args[1].clone(), env, &local_env_overwrites)
            }
            _ => { Err(EvaluationError::WrongType("Second argument of let binding must be array".parse().unwrap())) }
        }
    } else {
        Err(EvaluationError::WrongArity(2, args.len() as i64))
    }
}

fn call_function(called_fn: FunctionType, args_fn_is_receiving: VecDeque<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    let evaled_args = util::map_m(args_fn_is_receiving.into_iter().map(|x| eval_expression(x, env, local_vars)).collect());

    match evaled_args {
        Ok(correctly_evaled_args) => {
            match called_fn {
                FunctionType::BuiltIn(built_in) => built_in(correctly_evaled_args),
                FunctionType::Lambda(params_fn_takes, body) => {
                    let two_together = params_fn_takes.into_iter().zip(correctly_evaled_args).collect();
                    let map_of_bound_names = vector_of_tuples_to_hash_map_symbol(&two_together, env, local_vars)?;
                    let local_env_overwrites = SinglyLinkedList::Cons(map_of_bound_names, local_vars);
                    eval_expression(*body, env, &local_env_overwrites)
                },
            }
        }
        Err(err) => Err(err)
    }
}


fn call_special_form(special_form: SpecialForm, args: VecDeque<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    match special_form {
        SpecialForm::Quote => quote_expr(args),
        SpecialForm::If => if_expr(args, env, local_vars),
        SpecialForm::Fn => lambda(args),
        SpecialForm::Def => def(args, env, local_vars),
        SpecialForm::Let => let_expr(args, env, local_vars)
    }
}

fn lookup_vars<'a>(sym: &String, vars_layers: &'a LocalVars) -> Option<&'a Expression> {
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

fn lookup_symbol(sym: &String, env: &Environment, local_vars: &LocalVars ) -> EvalResult<Expression> {
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

fn eval_array(array: &Vec<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    let evaled_array = array.into_iter().map(|x| eval_expression(x.clone(), env, local_vars)).collect();
    match util::map_m(evaled_array) {
        Ok(correctly_evaled_array) => { Ok (Expression::Array(correctly_evaled_array))}
        Err(err) => Err(err)
    }
}

fn eval_map(map: &MapType, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    match map {
        MapType::PostEvaluation(_) => { Ok(Expression::Map(map.clone())) }
        MapType::PreEvaluation(tuples) => {
            let correctly_evaled_map = vector_of_tuples_to_hash_map_atom(&tuples, env, local_vars)?;
            Ok(Expression::Map(MapType::PostEvaluation(correctly_evaled_map)))
        }
    }
}

fn eval_list(list_elements: &VecDeque<Expression>, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    // TODO: This is a bit bad, I'd rather avoid mutability and clones and shit
    let mut here = list_elements.clone();

    if here.len() == 0 {
        Ok(Expression::List(here))
    } else {
        let caller = eval_expression(here[0].clone(), env, local_vars)?;
        here.pop_front();
        match caller {
            Expression::SpecialForm(special_form_type) => call_special_form(special_form_type.clone(), here, env, local_vars),
            Expression::Function(fn_type) => call_function(fn_type, here, env, local_vars),
            _ => Err(EvaluationError::NotSpecialFormOrFunction)
        }
    }
}

fn eval_expression(expr: Expression, env: &mut Environment, local_vars: &LocalVars) -> EvalResult<Expression> {
    match expr {
        Expression::At(Atom::Symbol(sym)) => lookup_symbol(&sym, env, local_vars),
        Expression::At(_) => Ok(expr),
        Expression::Function(_) => Ok(expr),
        Expression::SpecialForm(_) => Err(EvaluationError::SpecialFormOutOfContext), // TODO: Impossible due to lookup
        Expression::Array(array) => eval_array(&array, env, local_vars),
        Expression::Map(map) => eval_map(&map, env, local_vars),
        Expression::List(list) => eval_list(&list, env, local_vars)
    }
}

pub fn eval(expr: Expression, env: &mut Environment, local_vars: &LocalVars) -> String {
    let evaled_expr = eval_expression(expr, env, local_vars);
    format!("{:?}", evaled_expr)
}
