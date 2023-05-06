mod parse;

use rustpython_parser::ast::Constant;

use crate::parse::parse_code;

fn main() {
    let code = "if a and b:\n x = '1'\n";
    let nodes = parse_code(code, None).unwrap();
    dbg!(nodes);
}

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    Range(i64),
    True,
    False,
    None,
}

#[derive(Clone, Debug)]
enum Operator {
    And,
    Or,
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Debug, Clone)]
enum Expr {
    Assign {
        target: String,
        value: Box<Expr>,
    },
    Constant(Constant),
    Name(String),
    Call {
        func: String,
        args: Vec<Expr>,
        kwargs: Vec<(String, Expr)>,
    },
    Op {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
enum Node {
    Pass,
    Expression(Expr),
    For {
        target: Expr,
        iter: Expr,
        body: Vec<Node>,
        or_else: Vec<Node>,
    },
    If {
        test: Expr,
        body: Vec<Node>,
        or_else: Vec<Node>,
    },
}

