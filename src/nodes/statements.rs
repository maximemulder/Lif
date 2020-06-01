use super::statement::Statement;
use super::Node;

pub struct Statements {
	statements: Vec<Statement>,
}

impl Node for Statements {
	fn execute(&self) {
		for statement in self.statements.iter() {
			statement.execute();
		}
	}
}
