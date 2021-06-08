use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Jump {
    None,
    Continue,
    Break,
    Return
}

impl fmt::Display for Jump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::None     => "None",
            Self::Continue => "Continue",
            Self::Break    => "Break",
            Self::Return   => "Return",
        })
    }
}
