use types::{EvalResult, EvaluationError};

pub (crate) fn map_m<Expected,PossibleError>(maybes: Vec<Result<Expected,PossibleError>>) -> Result<Vec<Expected>, PossibleError> {
    let mut to_return = Vec::new();
    for maybe_err in maybes {
        match maybe_err {
            Err(err) => return Err(err),
            Ok(ok) => to_return.push(ok)
        }
    }
    Ok(to_return)
}

pub (crate) fn tuple_even_vector<T>(to_tuple: Vec<T>) -> EvalResult<Vec<(T, T)>> {
    let mut ret = Vec::new();
    let mut intermediate_tuple: Option<T> = Option::None;
    for half_binding in to_tuple {
        match intermediate_tuple {
            Option::Some(let1) => { ret.push((let1, half_binding)); intermediate_tuple = Option::None }
            Option::None => { intermediate_tuple = Option::Some(half_binding) }
        }
    }
    match intermediate_tuple {
        Option::Some(_) => Err(EvaluationError::MustHaveEvenNumberOfForms("let binding".parse().unwrap())),
        Option::None => { Ok(ret) }
    }
}
