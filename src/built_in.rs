use parser::{Expression, Atom, EvaluationError};
use parser::EvalResult;

pub fn sum(args: Vec<Expression>) -> EvalResult {
    // TODO: Upgrade to Nightly Rust to use fold_first
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
                            _NotInt => Err(EvaluationError::WrongType("atom but not integer".parse().unwrap()))
                        }
                    }
                    _NotAtom => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
                }
            }
            _NotAccAtom_ImpossibleWhichIsWhyWeNeedFoldFirst => Err(EvaluationError::WrongType("not atom".parse().unwrap()))
        }
    });
    fold_res
}
