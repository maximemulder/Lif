use crate::runtime::engine::Engine;
use crate::runtime::scope::{ GcScope, Scope };

impl<'a> Engine<'a> {
    pub fn run_scope<T>(&mut self, callback: &dyn Fn(&mut Engine<'a>) -> T) -> T {
        self.push_scope();
        let r#return = callback(self);
        self.pop_scope();
        r#return
    }

    pub fn run_frame<T>(&mut self, scope: GcScope<'a>, callback: &dyn Fn(&mut Engine<'a>) -> T) -> T {
        self.push_frame(scope);
        let r#return = callback(self);
        self.pop_frame();
        r#return
    }
}

impl<'a> Engine<'a> {
    pub fn new_scope(&mut self) -> GcScope<'a> {
        self.alloc(Scope::new(Some(self.scope)))
    }

    pub fn scope(&self) -> GcScope<'a> {
        self.scope
    }
}

impl<'a> Engine<'a> {
    fn push_scope(&mut self) {
        self.scope = self.new_scope();
    }

    fn pop_scope(&mut self) {
        self.scope = self.scope.parent().unwrap();
    }

    fn push_frame(&mut self, frame: GcScope<'a>) {
        self.frames.push(self.scope);
        self.scope = frame;
    }

    fn pop_frame(&mut self) {
        self.scope = self.frames.pop().unwrap();
    }
}
