#[derive(PartialEq, Eq)]
pub struct Element {
    pub name: &'static str,
}

impl Element {
    pub const fn new(name: &'static str) -> Self {
        return Self {
            name,
        };
    }
}
