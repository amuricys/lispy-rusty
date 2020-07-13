use std::collections::HashMap;

#[derive(Debug, Clone)]

/* TODO: Discover how to parametrize the our types.
   For example: an Expression::Function(vec, _) doesn't contain, in vec, just any atom.
   In fact its atoms should be Symbols that we can easily add to the fn's local environment.

   Another example: An Ok(expr) isn't just any expression.
   E.g.:  When evaluation of summation goes OK, the result is not
   just any Expression, it is in fact an Expression that contains an integer.
   Any value of that return Expression is going to successfully pattern match on
   the pattern
   Expression::At(Atom::Int(val))

   How do we get this guarantee on the type level? */
pub enum Atom {
    Int(i64),
    Char(char),
    Bool(bool),
    Nil,
    Symbol(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    At(Atom),
    Expr(Box<Expression>, Vec<Expression>), // TODO also: implement this as a linked list (which it kind of already is but very buffed)
    Array(Vec<Expression>),
    SpecialForm(SpecialForm),
    /* TODO: Perhaps instead of having the two below, we could have a single "Function" enum that is either built-in
    or is a lambda. In both cases they need to be aware of their `env`ironment, and need to evaluate the args before called. */
    BuiltIn(fn(Vec<Expression>) -> EvalResult),
    Lambda(Vec<Atom>, Box<Expression>)

}

#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    SymbolNotFound,
    NotSpecialFormOrFunction,
    Pending,
    WrongType(String),
    WrongArity(i64, i64), // (Expected, Received)
    SpecialFormOutOfContext,
}

pub type EvalResult = Result<Expression, EvaluationError>;
/* ------------------------------------------------------------------------ */

#[derive (Debug, Clone)]
pub enum SpecialForm {
    /*  (QExpression) */
    Quote,
    /*  (Cond, If Branch, Else Branch) */
    If,
    /*  (ArgList, Body) */
    Fn,
    /*  (Name, Value) */
    Def,
}

#[derive (Debug, Clone)]
pub struct Environment {
    pub special_forms: HashMap<String, SpecialForm>,
    pub built_in_fns: HashMap<String, fn(Vec<Expression>) -> EvalResult>,
    pub vars: HashMap<String, Expression>
}

