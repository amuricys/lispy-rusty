use parser::EvalResult;
use parser::{Atom, EvaluationError, Expression};

pub fn neg(args: Vec<Expression>) -> EvalResult {
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

pub fn sum2(args: Vec<Expression>) -> EvalResult {
    let mut acc = 0;
    for arg in args {
        if let Expression::At(atom) = arg {
            if let Atom::Int(value) = atom {
                acc += value
            } else {
                return Err(EvaluationError::WrongType(
                    "atom but not integer".parse().unwrap(),
                ));
            }
        } else {
            return Err(EvaluationError::WrongType("not atom".parse().unwrap()));
        }
    }
    Ok(Expression::At(Atom::Int(acc)))
}

pub fn sub2(args: Vec<Expression>) -> EvalResult {
    let mut first_value = true;
    let mut sbb = 0;
    for arg in args {
        if let Expression::At(atom) = arg {
            if let Atom::Int(value) = atom {
                sbb = if first_value {
                    first_value = false;
                    value
                } else {
                    sbb - value
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
    Ok(Expression::At(Atom::Int(sbb)))
}

pub fn mult2(args: Vec<Expression>) -> EvalResult {
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

pub fn div2(args: Vec<Expression>) -> EvalResult {
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

pub fn sum(args: Vec<Expression>) -> EvalResult {
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

pub fn div(args: Vec<Expression>) -> EvalResult {
    if let Expression::At(Atom::Int(first_elem)) = args[0] {
        let fold_res = args.into_iter().fold(
            Ok(Expression::At(Atom::Int(first_elem))),
            |acc, arg| match acc {
                Err(err) => Err(err),
                Ok(Expression::At(Atom::Int(acc_val))) => match arg {
                    Expression::At(atom) => match atom {
                        Atom::Int(atom_val) => {
                            if atom_val == 0 {
                                Err(EvaluationError::DivideByZero)
                            } else {
                                Ok(Expression::At(Atom::Int(acc_val / atom_val)))
                            }
                        }
                        _ => Err(EvaluationError::WrongType(
                            "atom but not integer".parse().unwrap(),
                        )),
                    },
                    _ => Err(EvaluationError::WrongType("not atom".parse().unwrap())),
                },
                _ => Err(EvaluationError::WrongType("not atom".parse().unwrap())),
            },
        );
        fold_res
    } else {
        println!("FUCK YOU BOAH");
        Err(EvaluationError::DivideByZero)
    }
}
