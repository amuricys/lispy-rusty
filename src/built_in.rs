use parser::{Expression, Atom, EvaluationError};
use parser::EvalResult;

pub fn neg(args: Vec<Expression>) -> EvalResult {
    let args_amount = args.len();
    if args_amount != 1 {
        Err(EvaluationError::WrongArity(1, args_amount as i64))
    } else {
        match &args[0] {
            Expression::At(atom) => {
                match atom {
                    Atom::Int(i) =>{ Ok(Expression::At(Atom::Int(i * (-1)))) }
                    _ => Err(EvaluationError::WrongType("atom but not integer".parse().unwrap()))
                }
            }
            _ => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
        }
    }
}

pub fn sum(args: Vec<Expression>) -> EvalResult {
    let fold_res = args.into_iter().fold(Ok(Expression::At(Atom::Int(0))), |acc, arg | {
        match acc {
            Err(err) => { Err(err) }
            Ok(Expression::At(Atom::Int(acc_val))) => {
                match arg {
                    Expression::At(atom) => {
                        match atom {
                            Atom::Int(atom_val) => {
                                Ok(Expression::At(Atom::Int(acc_val + atom_val)))
                            }
                            _ => Err(EvaluationError::WrongType("atom but not integer".parse().unwrap()))
                        }
                    }
                    _ => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
                }
            }
            _ => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
        }
    });
    fold_res
}

pub fn div(args: Vec<Expression>) -> EvalResult {
    let first_elem = args[0].clone();
    let fold_res = args.into_iter().fold(Ok(first_elem), |acc, arg | {
        match acc {
            Err(err) => { Err(err) }
            Ok(Expression::At(Atom::Int(acc_val))) => {
                match arg {
                    Expression::At(atom) => {
                        match atom {
                            Atom::Int(atom_val) => {
                                if atom_val == 0 { Err(EvaluationError::DivideByZero) }
                                else { Ok(Expression::At(Atom::Int(acc_val / atom_val))) }
                            }
                            _ => Err(EvaluationError::WrongType("atom but not integer".parse().unwrap()))
                        }
                    }
                    _ => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
                }
            }
            _ => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
        }
    });
    fold_res
}
