#[derive(PartialEq, Eq)]
pub enum Control {
    Continue,
    Break,
    Return
}

impl Control {
    fn is_loop(&self) -> bool {
        match self {
            Control::Continue => true,
            Control::Break    => true,
            Control::Return   => false,
        }
    }
}
