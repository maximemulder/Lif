pub trait GcTrace {
    fn trace(&mut self) {}
}

impl GcTrace for () {}
