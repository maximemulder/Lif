#[derive(PartialEq, Eq)]
pub enum Jump {
    None,
    Continue,
    Break,
    Return
}

impl Jump {
    fn is_loop(&self) -> bool {
        match self {
            Self::None     => true,
            Self::Continue => true,
            Self::Break    => true,
            Self::Return   => false,
        }
    }
}
