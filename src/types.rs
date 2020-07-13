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
    Array(Vec<Expression>), // TODO: Implement Fn as a special form that validates its arguments
    Function(Vec<Atom>, Box<Expression>)
}

#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    SymbolNotFound,
    NotAFunction,
    Pending,
    WrongType(String),
    WrongArity(i64, i64), // (Expected, Received)
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
pub enum ResolvedSymbol {
    Value(Expression),
    SpecialF(SpecialForm),
}
