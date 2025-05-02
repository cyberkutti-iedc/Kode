use std::fmt;
use serde::{Serialize, Deserialize};

/// Top-level program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Program {
    Statements(Vec<Statement>),
}

/// Statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let { name: String, value: Expression },
    Assign { name: String, value: Expression },
    FunctionDef { 
        file_prefix: String,
        is_main: bool,
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Expression),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While { condition: Expression, body: Vec<Statement> },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Box<Statement>>,
        body: Vec<Statement>,
    },
    ExprStmt(Expression),
    Block(Vec<Statement>),
    Print(Expression),
    Expression(Expression),
    Import(String),
    Try {
        body: Vec<Statement>,
        catch: Vec<Statement>,
    },
}

/// Expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Array {
        elements: Vec<Expression>,
    },
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Closure {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",
            BinaryOp::LessThan => "<",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessThanOrEqual => "<=",
            BinaryOp::GreaterThanOrEqual => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        };
        write!(f, "{}", s)
    }
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate, // -
    Not,    // !
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Negate => "-",
            UnaryOp::Not => "!",
        };
        write!(f, "{}", s)
    }
}

/// Represent a type in the language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Array(Box<Type>),
    Void,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}
