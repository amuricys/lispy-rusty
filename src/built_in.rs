use types::{Expression, EvaluationError, EvalResult, Atom, Environment, SpecialForm, SinglyLinkedList};
use std::collections::HashMap;

// TODO: Built-in fns estão pedreiras
fn neg(args: Vec<Expression>) -> EvalResult<Expression> {
    let args_amount = args.len();
    if args_amount != 1 {
        Err(EvaluationError::WrongArity(1, args_amount as i64))
    } else {
        match &args[0] {
            Expression::At(atom) => match atom {
                Atom::Int(i) => Ok(Expression::At(Atom::Int(i * (-1)))),
                _ => Err(EvaluationError::WrongType(
                    "atom but not integer".parse().unwrap(),
                )),
            },
            _ => Err(EvaluationError::WrongType("not atom".parse().unwrap())),
        }
    }
}

fn mul(args: Vec<Expression>) -> EvalResult<Expression> {
    let mut mtt = 1;
    for arg in args {
        if let Expression::At(atom) = arg {
            if let Atom::Int(value) = atom {
                mtt *= value;
            } else {
                return Err(EvaluationError::WrongType(
                    "atom but not integer".parse().unwrap(),
                ));
            }
        } else {
            return Err(EvaluationError::WrongType("not atom".parse().unwrap()));
        }
    }
    Ok(Expression::At(Atom::Int(mtt)))
}

fn div(args: Vec<Expression>) -> EvalResult<Expression> {
    let mut first_value = true;
    let mut dvv = 0;
    for arg in args {
        if let Expression::At(atom) = arg {
            if let Atom::Int(value) = atom {
                if value == 0 {
                    return Err(EvaluationError::DivideByZero);
                }
                dvv = if first_value {
                    first_value = false;
                    value
                } else {
                    dvv / value
                }
            } else {
                return Err(EvaluationError::WrongType(
                    "atom but not integer".parse().unwrap(),
                ));
            }
        } else {
            return Err(EvaluationError::WrongType("not atom".parse().unwrap()));
        }
    }
    Ok(Expression::At(Atom::Int(dvv)))
}

fn sum(args: Vec<Expression>) -> EvalResult<Expression> {
    args.into_iter()
        .fold(Ok(Expression::At(Atom::Int(0))), |acc, arg| match acc {
            Err(err) => Err(err),
            Ok(Expression::At(Atom::Int(acc_val))) => match arg {
                Expression::At(atom) => match atom {
                    Atom::Int(atom_val) => Ok(Expression::At(Atom::Int(acc_val + atom_val))),
                    _ => Err(EvaluationError::WrongType(
                        "atom but not integer".parse().unwrap(),
                    )),
                },
                _ => Err(EvaluationError::WrongType("not atom".parse().unwrap())),
            },
            _ => Err(EvaluationError::WrongType("not atom".parse().unwrap())),
        })
}

pub fn initial_env() -> Environment {
    let mut special_forms = HashMap::<String, SpecialForm>::new();
    let mut built_ins = HashMap::<String, fn(Vec<Expression>) -> EvalResult<Expression>>::new();
    let vars = HashMap::<String, Expression>::new();

    built_ins.insert("+".to_string(), sum);
    built_ins.insert("*".to_string(), mul);
    built_ins.insert("/".to_string(), div);
    built_ins.insert("neg".to_string(), neg);

    special_forms.insert("quote".to_string(), SpecialForm::Quote);
    special_forms.insert("if".to_string(), SpecialForm::If);
    special_forms.insert("fn".to_string(), SpecialForm::Fn);
    special_forms.insert("def".to_string(), SpecialForm::Def);

    Environment {
        special_forms: special_forms,
        built_in_fns: built_ins,
        top_level_vars: vars,
    }
}
