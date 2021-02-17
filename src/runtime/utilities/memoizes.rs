use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::value::GcValue;

pub struct Memoizes<'a> {
    memoizes: Vec<Memoize<'a>>,
}

impl<'a> Memoizes<'a> {
    pub fn new() -> Self {
        Self {
            memoizes: Vec::new(),
        }
    }

    pub fn record(&mut self, arguments: Arguments<'a>, reference: GcReference<'a>) {
        self.memoizes.push(Memoize::new(arguments, reference));
    }

    pub fn get(&self, values: &[GcValue<'a>]) -> Option<GcReference<'a>> {
        for memoize in self.memoizes.iter() {
            if let Some(reference) = memoize.get(values) {
                return Some(reference);
            }
        }

        None
    }
}

impl GcTrace for Memoizes<'_> {
    fn trace(&mut self) {
        for memoize in self.memoizes.iter_mut() {
            memoize.trace();
        }
    }
}

struct Memoize<'a> {
    arguments: Arguments<'a>,
    reference: GcReference<'a>,
}

impl<'a> Memoize<'a> {
    fn new(arguments: Arguments<'a>, reference: GcReference<'a>) -> Self {
        Self {
            arguments,
            reference,
        }
    }

    fn get(&self, values: &[GcValue<'a>]) -> Option<GcReference<'a>> {
        for (argument, value) in self.arguments.iter().zip(values) {
            if !value.is(*argument) {
                return None;
            }
        }

        Some(self.reference)
    }
}

impl GcTrace for Memoize<'_> {
    fn trace(&mut self) {
        self.reference.trace();
        for argument in self.arguments.iter_mut() {
            argument.trace()
        }
    }
}
