use crate::memory::Ref;
use crate::parser::{ Code, Grammar, SNode };
use crate::parser::arena::ArenaRef;
use crate::parser::ascent::*;
use crate::parser::descent::*;

use std::cmp;

pub struct Parse<'a> {
    grammar: &'a Grammar,
    pub code: Ref<Code>,
    tokens: &'a [SNode],
    cursor: usize,
    reach: usize,
}

impl<'a> Parse<'a> {
    pub fn new(grammar: &'a Grammar, code: Ref<Code>, tokens: &'a [SNode]) -> Self {
        Self {
            grammar,
            code,
            tokens,
            cursor: 0,
            reach: 0,
        }
    }

    fn done(&self) -> bool {
        self.cursor == self.tokens.len()
    }

    pub fn next(&mut self) -> Option<SNode> {
        let option = self.tokens.get(self.cursor);
        if let Some(token) = option {
            if self.reach < self.cursor {
                self.reach = self.cursor;
            }

            self.cursor += 1;
            return Some(token.clone());
        }

        None
    }

    fn run(&mut self, callback: impl FnOnce(&mut Self) -> Option<Vec<SNode>>) -> Option<Vec<SNode>>{
        let cursor = self.cursor;
        let nodes = callback(self);
        if nodes.is_none() {
            self.cursor = cursor;
        }

        nodes
    }

    fn run_predicate(&mut self, callback: impl FnOnce(&mut Self) -> Option<Vec<SNode>>) -> bool {
        let cursor = self.cursor;
        let nodes = callback(self);
        self.cursor = cursor;
        nodes.is_some()
    }

    pub fn descent(&mut self, r#ref: ArenaRef<dyn Descent>) -> Option<Vec<SNode>> {
        self.run(|parse| parse.grammar.descents.get(r#ref).descent(parse))
    }

    pub fn descent_predicate(&mut self, r#ref: ArenaRef<dyn Descent>) -> bool {
        self.run_predicate(|parse| parse.grammar.descents.get(r#ref).descent(parse))
    }

    pub fn ascent(&mut self, r#ref: ArenaRef<dyn Ascent>, nodes: Vec<SNode>) -> Option<Vec<SNode>> {
        self.run(|parse| parse.grammar.ascents.get(r#ref).ascent(parse, nodes))
    }

    pub fn ascent_predicate(&mut self, r#ref: ArenaRef<dyn Ascent>, nodes: Vec<SNode>) -> bool {
        self.run_predicate(|parse| parse.grammar.ascents.get(r#ref).ascent(parse, nodes))
    }

    pub fn parse(&mut self, production: ArenaRef<dyn Descent>) -> Option<SNode> {
        let node = if let Some(mut nodes) = self.grammar.descents.get(production).descent(self) {
            nodes.pop()
        } else {
            return None;
        };

        if self.done() {
            node
        } else {
            let token = &self.tokens[self.reach];
            println!("PARSING ERROR, LINE {}, POSITION {}, UNEXPECTED TOKEN: {:?} - {}\n\n{}\n{}{}",
                self.code.node_y(token),
                self.code.node_x(token),
                self.code.node_str(token),
                token.element.name,
                self.code.node_line(token),
                " ".repeat(self.code.node_shift_left(token)),
                "^".repeat(cmp::min(self.code.node_str(token).len(), self.code.node_shift_right(token)))
            );

            None
        }
    }
}
