use std::collections::HashMap;

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
#[derive(Debug, Clone)]
pub enum Atom {
    Int(i64),
    Char(char),
    Bool(bool),
    Nil,
    Symbol(String),
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Lambda(Vec<Atom>, Box<Expression>),
    BuiltIn(fn(Vec<Expression>) -> EvalResult<Expression>)
}

#[derive(Debug, Clone)]
pub enum Expression {
    At(Atom),
    List(Box<Expression>, Vec<Expression>), // TODO: implement this as a linked list (which it kind of already is but very buffed)
    Array(Vec<Expression>),
    SpecialForm(SpecialForm),
    Function(FunctionType)
}

#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    SymbolNotFound(String),
    NotSpecialFormOrFunction,
    Pending,
    WrongType(String),
    WrongArity(i64, i64), // (Expected, Received)
    SpecialFormOutOfContext,
    UnknownBinding(String)
}

pub type EvalResult<O> = Result<O, EvaluationError>;
/* ------------------------------------------------------------------------ */

#[derive (Debug, Clone)]
pub enum SpecialForm {
    Quote,
    If,
    Fn,
    Def,
}

#[derive (Debug, Clone)]
// TODO: Implement as linked list also, so that incremental additions to environments are possible
pub struct Environment {
    pub special_forms: HashMap<String, SpecialForm>,
    pub built_in_fns: HashMap<String, fn(Vec<Expression>) -> EvalResult<Expression>>,
    pub vars: HashMap<String, Expression>
}

