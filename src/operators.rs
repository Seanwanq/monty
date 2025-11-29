use std::fmt;

use strum::Display;

/// Binary operators for arithmetic, bitwise, and boolean operations.
///
/// Uses strum `Display` derive with per-variant serialization for operator symbols.
#[derive(Clone, Debug, PartialEq, Display)]
pub(crate) enum Operator {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Sub,
    #[strum(serialize = "*")]
    Mult,
    #[strum(serialize = "@")]
    MatMult,
    #[strum(serialize = "/")]
    Div,
    #[strum(serialize = "%")]
    Mod,
    #[strum(serialize = "**")]
    Pow,
    #[strum(serialize = "<<")]
    LShift,
    #[strum(serialize = ">>")]
    RShift,
    #[strum(serialize = "|")]
    BitOr,
    #[strum(serialize = "^")]
    BitXor,
    #[strum(serialize = "&")]
    BitAnd,
    #[strum(serialize = "//")]
    FloorDiv,
    // bool operators
    #[strum(serialize = "and")]
    And,
    #[strum(serialize = "or")]
    Or,
}

/// Defined separately since these operators always return a bool
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CmpOperator {
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
    // we should support floats too, either via a Number type, or ModEqInt and ModEqFloat
    ModEq(i64),
}

impl fmt::Display for CmpOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::LtE => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::GtE => write!(f, ">="),
            Self::Is => write!(f, "is"),
            Self::IsNot => write!(f, "is not"),
            Self::In => write!(f, "in"),
            Self::NotIn => write!(f, "not in"),
            Self::ModEq(v) => write!(f, "% X == {v}"),
        }
    }
}
