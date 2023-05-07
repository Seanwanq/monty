use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Operator {
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
    // bool operators
    And,
    Or,
    // compare
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Debug, Clone)]
pub(crate) enum Expr<T, Funcs> {
    Constant(Value),
    Name(T),
    Call {
        func: Funcs,
        args: Vec<Expr<T, Funcs>>,
        // kwargs: Vec<(T, Expr<T, Funcs>)>,
    },
    Op {
        left: Box<Expr<T, Funcs>>,
        op: Operator,
        right: Box<Expr<T, Funcs>>,
    },
    List(Vec<Expr<T, Funcs>>),
}

#[derive(Debug, Clone)]
pub(crate) enum Node<Vars, Funcs> {
    Pass,
    Expr(Expr<Vars, Funcs>),
    Assign {
        target: Vars,
        value: Box<Expr<Vars, Funcs>>,
    },
    For {
        target: Expr<Vars, Funcs>,
        iter: Expr<Vars, Funcs>,
        body: Vec<Node<Vars, Funcs>>,
        or_else: Vec<Node<Vars, Funcs>>,
    },
    If {
        test: Expr<Vars, Funcs>,
        body: Vec<Node<Vars, Funcs>>,
        or_else: Vec<Node<Vars, Funcs>>,
    },
}

// this is a temporary hack
#[derive(Debug, Clone)]
pub(crate) enum Builtins {
    Print,
    Range,
}

impl Builtins {
    pub fn find(name: &str) -> crate::prepare::PrepareResult<Self> {
        match name {
            "print" => Ok(Self::Print),
            "range" => Ok(Self::Range),
            _ => Err(format!("unknown function: {}", name).into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Undefined,
    Ellipsis,
    None,
    True,
    False,
    Int(i64),
    Bytes(Vec<u8>),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Range(i64),
}

fn format_iterable(start: char, end: char, items: &[Value], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", start)?;
    let mut items_iter = items.iter();
    if let Some(first) = items_iter.next() {
        write!(f, "{first}")?;
        for item in items_iter {
            write!(f, ", {item}")?;
        }
    }
    write!(f, "{}", end)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => write!(f, "Undefined"),
            Self::Ellipsis => write!(f, "..."),
            Self::None => write!(f, "None"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Str(v) => write!(f, "{v}"),
            Self::Bytes(v) => write!(f, "{v:?}"), // TODO: format bytes
            Self::List(v) => format_iterable('[', ']', v, f),
            Self::Tuple(v) => format_iterable('(', ')', v, f),
            Self::Range(size) => write!(f, "0:{size}"),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Value {
    pub fn add(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => Some(Self::Int(v1 + v2)),
            (Self::Str(mut v1), Self::Str(v2)) => {
                v1.push_str(&v2);
                Some(Self::Str(v1))
            }
            (Self::List(mut v1), Self::List(v2)) => {
                v1.extend(v2);
                Some(Self::List(v1))
            }
            _ => None,
        }
    }

    pub fn sub(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => Some(Self::Int(v1 - v2)),
            _ => None,
        }
    }

    pub fn eq(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Undefined, _) => None,
            (_, Self::Undefined) => None,
            (Self::Int(v1), Self::Int(v2)) => Self::_true_false(v1 == v2),
            (Self::Str(v1), Self::Str(v2)) => Self::_true_false(v1 == v2),
            (Self::List(v1), Self::List(v2)) => {
                if v1.len() != v2.len() {
                    Some(Self::False)
                } else {
                    for (v1, v2) in v1.into_iter().zip(v2.into_iter()) {
                        if let Some(v) = v1.eq(v2) {
                            if v == Self::False {
                                return Some(Self::False);
                            }
                        } else {
                            return None;
                        }
                    }
                    Some(Self::True)
                }
            }
            (Self::Range(v1), Self::Range(v2)) => Self::_true_false(v1 == v2),
            (Self::True, Self::True) => Some(Self::True),
            (Self::True, Self::Int(v2)) => Self::_true_false(1 == v2),
            (Self::Int(v1), Self::True) => Self::_true_false(v1 == 1),
            (Self::False, Self::False) => Some(Self::True),
            (Self::False, Self::Int(v2)) => Self::_true_false(0 == v2),
            (Self::Int(v1), Self::False) => Self::_true_false(v1 == 0),
            (Self::None, Self::None) => Some(Self::True),
            _ => Some(Self::False),
        }
    }

    pub fn bool(&self) -> Option<bool> {
        match self {
            Self::Undefined => None,
            Self::Ellipsis => Some(true),
            Self::None => Some(false),
            Self::True => Some(true),
            Self::False => Some(false),
            Self::Int(v) => Some(*v != 0),
            Self::Float(f) => Some(*f != 0.0),
            Self::Str(v) => Some(!v.is_empty()),
            Self::Bytes(v) => Some(!v.is_empty()),
            Self::List(v) => Some(!v.is_empty()),
            Self::Tuple(v) => Some(!v.is_empty()),
            Self::Range(v) => Some(*v != 0),
        }
    }

    pub fn invert(&self) -> Option<Value> {
        match self {
            Self::True => Some(Self::False),
            Self::False => Some(Self::True),
            _ => None,
        }
    }

    fn _true_false(v: bool) -> Option<Self> {
        if v {
            Some(Self::True)
        } else {
            Some(Self::False)
        }
    }
}
