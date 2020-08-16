use std::collections::{VecDeque, HashMap};

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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Atom {
    Int(i64),
    Char(char),
    Bool(bool),
    Nil,
    Symbol(String),
    String(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionType {
    Lambda(Vec<Expression>, Box<Expression>),
    BuiltIn(fn(Vec<Expression>) -> EvalResult<Expression>)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MapType {
    PreEvaluation(Vec<(Expression, Expression)>),
    PostEvaluation(HashMap<Atom, Expression>)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    At(Atom),
    List(VecDeque<Expression>),
    Array(Vec<Expression>),
    Map(MapType),
    SpecialForm(SpecialForm),
    Function(FunctionType)
}

// TODO: Make all evaluation errors return strings
#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    SymbolNotFound(String),
    NotSpecialFormOrFunction,
    Pending,
    WrongType(String),
    WrongArity(i64, i64), // (Expected, Received)
    SpecialFormOutOfContext,
    UnknownBinding(String),
    ForbiddenDef(String),
    MustHaveEvenNumberOfForms(String)
}

pub type EvalResult<O> = Result<O, EvaluationError>;
/* ------------------------------------------------------------------------ */

#[derive (Debug, Clone, Eq, PartialEq)]
pub enum SpecialForm {
    Quote,
    If,
    Fn,
    Def,
    Let,
}

#[derive (Debug, Eq, PartialEq)]
pub enum SinglyLinkedList<'a, T> {
    Cons(T, &'a SinglyLinkedList<'a, T>),
    Nil
}

#[derive (Debug, Clone)]
pub struct Environment {
    pub special_forms: HashMap<String, SpecialForm>,
    pub built_in_fns: HashMap<String, fn(Vec<Expression>) -> EvalResult<Expression>>,
    pub top_level_vars: HashMap<String, Expression>
}

