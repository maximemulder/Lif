use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::value::Value;

impl<'a> Engine<'a> {
    pub fn scope(&self) -> GcScope<'a> {
        self.scope
    }

    pub fn run_source_scope(&mut self, name: &str, callback: impl FnOnce(&mut Engine<'a>, GcScope<'a>) -> Value<'a>) -> Value<'a> {
        let mut scope = self.new_scope();
        let value = callback(self, scope);
        scope.set_source(self, name, value);
        value
    }

    pub fn run_scope<T>(&mut self, callback: impl FnOnce(&mut Engine<'a>) -> T) -> T {
        self.push_scope();
        let r#return = callback(self);
        self.pop_scope();
        r#return
    }

    pub fn run_frame<T>(&mut self, frame: GcScope<'a>, callback: impl FnOnce(&mut Engine<'a>) -> T) -> T {
        self.push_frame(frame);
        let r#return = callback(self);
        self.pop_frame();
        r#return
    }


    pub fn run_gc(&mut self, callback: impl FnOnce(&mut Engine<'a>) -> ReturnFlow<'a>) -> ReturnFlow<'a> {
        self.cache.push();
        let r#return = callback(self);
        if let Ok(flow) = r#return.as_ref() {
            self.cache.bubble(flow.reference);
        }

        self.cache.pop();
        r#return
    }
}

impl<'a> Engine<'a> {
    fn new_scope(&mut self) -> GcScope<'a> {
        self.alloc(Scope::new(Some(self.scope)))
    }

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
