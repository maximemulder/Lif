trait Node<'a> {
	fn children(&self) -> Box<[&'a dyn Node]>;
}

struct Token<'a> {
	element: &'a str,
}

impl<'a> Node<'a> for Token<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([]);
	}
}

struct Program<'a> {
	statements: Statements<'a>,
}

impl<'a> Node<'a> for Program<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.statements]);
	}
}

struct Statements<'a> {
	statements: Vec<Statement<'a>>,
}

impl<'a> Node<'a> for Statements<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut statements: Vec<&dyn Node> = Vec::new();
		for statement in self.statements.iter() {
			statements.push(statement);
		}

		return statements.into_boxed_slice();
	}
}

enum Statement<'a> {
	Expression(Box<StatementExpression<'a>>),
	Structure(Box<StatementStructure<'a>>),
}

impl<'a> Node<'a> for Statement<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([match self {
			Self::Expression(expression) => expression.as_ref(),
			Self::Structure(structure) => structure.as_ref(),
		}]);
	}
}

struct StatementExpression<'a> {
	expression: Expression<'a>,
	semicolon: Token<'a>
}

impl<'a> Node<'a> for StatementExpression<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.expression, &self.semicolon]);
	}
}

struct StatementStructure<'a> {
	structure: Structure<'a>,
	semicolon: Option<Box<Token<'a>>>
}

impl<'a> Node<'a> for StatementStructure<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.structure];
		if let Some(semicolon) = &self.semicolon {
			children.push(semicolon.as_ref());
		}

		return children.into_boxed_slice();
	}
}

enum Expression<'a> {
	Literal(Box<Literal<'a>>),
	Group(Box<Group<'a>>),
	Declaration(Box<Declaration<'a>>),
	Function(Box<Function<'a>>),
	Control(Box<Control<'a>>),
	Structure(Box<Structure<'a>>),
	Sequence(Box<Sequence<'a>>),
	Chain(Box<Chain<'a>>),
	Operation(Box<Operation<'a>>),
}

impl<'a> Node<'a> for Expression<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([match self {
			Self::Literal(literal) => literal.as_ref(),
			Self::Group(group) => group.as_ref(),
			Self::Declaration(declaration) => declaration.as_ref(),
			Self::Function(function) => function.as_ref(),
			Self::Control(control) => control.as_ref(),
			Self::Structure(structure) => structure.as_ref(),
			Self::Sequence(sequence) => sequence.as_ref(),
			Self::Chain(chain) => chain.as_ref(),
			Self::Operation(operation) => operation.as_ref(),
		}]);
	}
}

struct Expressions<'a> {
	content: Option<Box<ExpressionsContent<'a>>>,
}

impl<'a> Node<'a> for Expressions<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = Vec::new();
		if let Some(content) = &self.content {
			children.push(content.as_ref());
		}

		return children.into_boxed_slice();
	}
}

struct ExpressionsContent<'a> {
	expression: Expression<'a>,
	more: Option<Box<ExpressionsMore<'a>>>,
}

impl<'a> Node<'a> for ExpressionsContent<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.expression];
		if let Some(more) = &self.more {
			children.push(more.as_ref());
		}

		return children.into_boxed_slice();
	}
}

struct ExpressionsMore<'a> {
	comma: Token<'a>,
	expression: Expression<'a>,
	more: Option<Box<ExpressionsMore<'a>>>,
}

impl<'a> Node<'a> for ExpressionsMore<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.comma, &self.expression];
		if let Some(more) = &self.more {
			children.push(more.as_ref());
		}

		return children.into_boxed_slice();
	}
}

struct Literal<'a> {
	literal: Token<'a>,
}

impl<'a> Node<'a> for Literal<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.literal]);
	}
}

struct Group<'a> {
	open: Token<'a>,
	expression: Expression<'a>,
	close: Token<'a>,
}

impl<'a> Node<'a> for Group<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.open, &self.expression, &self.close]);
	}
}

struct Declaration<'a> {
	r#let: Token<'a>,
	identifier: Token<'a>,
}

impl<'a> Node<'a> for Declaration<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.r#let, &self.identifier]);
	}
}

struct Function<'a> {
	function: Token<'a>,
	open: Token<'a>,
	parameters: Parameters<'a>,
	close: Token<'a>,
	block: Block<'a>,
}

impl<'a> Node<'a> for Function<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.function, &self.open, &self.parameters, &self.close, &self.block]);
	}
}

struct Control<'a> {
	control: Token<'a>,
	expression: Option<Box<Expression<'a>>>,
}

impl<'a> Node<'a> for Control<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.control];
		if let Some(expression) = &self.expression {
			children.push(expression.as_ref());
		}

		return children.into_boxed_slice();
	}
}

struct Parameters<'a> {
	content: Option<Box<ParametersContent<'a>>>,
}

impl<'a> Node<'a> for Parameters<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = Vec::new();
		if let Some(content) = &self.content {
			children.push(content.as_ref());
		}

		return children.into_boxed_slice();
	}
}

struct ParametersContent<'a> {
	parameter: Token<'a>,
	more: Option<Box<ParametersMore<'a>>>,
}

impl<'a> Node<'a> for ParametersContent<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.parameter];
		if let Some(more) = &self.more {
			children.push(more.as_ref());
		}

		return children.into_boxed_slice();
	}
}

struct ParametersMore<'a> {
	comma: Token<'a>,
	parameter: Token<'a>,
	more: Option<Box<ParametersMore<'a>>>,
}

impl<'a> Node<'a> for ParametersMore<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.comma, &self.parameter];
		if let Some(more) = &self.more {
			children.push(more.as_ref());
		}

		return children.into_boxed_slice();
	}
}

enum Structure<'a> {
	Block(Box<Block<'a>>),
	If(Box<If<'a>>),
	Loop(Box<Loop<'a>>),
	While(Box<While<'a>>),
	For(Box<For<'a>>),
}

impl<'a> Node<'a> for Structure<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([match self {
			Self::Block(block) => block.as_ref(),
			Self::If(r#if) => r#if.as_ref(),
			Self::Loop(r#loop) => r#loop.as_ref(),
			Self::While(r#while) => r#while.as_ref(),
			Self::For(r#for) => r#for.as_ref(),
		}]);
	}
}

struct Block<'a> {
	open: Token<'a>,
	statements: Statements<'a>,
	expression: Option<Box<Expression<'a>>>,
	close: Token<'a>,
}

impl<'a> Node<'a> for Block<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.open, &self.statements];
		if let Some(expression) = &self.expression {
			children.push(expression.as_ref())
		}

		children.push(&self.close);
		return children.into_boxed_slice();
	}
}

struct If<'a> {
	r#if: Token<'a>,
	condition: Expression<'a>,
	body: Block<'a>,
	r#else: Option<Box<Else<'a>>>,
}

impl<'a> Node<'a> for If<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		let mut children: Vec<&dyn Node> = vec![&self.r#if, &self.condition, &self.body];
		if let Some(r#else) = &self.r#else {
			children.push(r#else.as_ref())
		}

		return children.into_boxed_slice();
	}
}

struct Else<'a> {
	r#else: Token<'a>,
	body: Block<'a>,
}

impl<'a> Node<'a> for Else<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.r#else, &self.body]);
	}
}

struct Loop<'a> {
	r#loop: Token<'a>,
	body: Block<'a>,
}

impl<'a> Node<'a> for Loop<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.r#loop, &self.body]);
	}
}

struct While<'a> {
	r#while: Token<'a>,
	condition: Expression<'a>,
	body: Block<'a>,
}

impl<'a> Node<'a> for While<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.r#while, &self.condition, &self.body]);
	}
}

struct For<'a> {
	r#for: Token<'a>,
	name: Token<'a>,
	r#in: Token<'a>,
	iterable: Expression<'a>,
	body: Block<'a>,
}

impl<'a> Node<'a> for For<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.r#for, &self.name, &self.r#in, &self.iterable, &self.body]);
	}
}

struct Chain<'a> {
	expression: Expression<'a>,
	dot: Token<'a>,
	identifier: Token<'a>,
}

impl<'a> Node<'a> for Chain<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.expression, &self.dot, &self.identifier]);
	}
}

struct Sequence<'a> {
	expression: Expression<'a>,
	open: Token<'a>,
	expressions: Expressions<'a>,
	close: Token<'a>,
}

impl<'a> Node<'a> for Sequence<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.expression, &self.open, &self.expressions, &self.close]);
	}
}

struct Operation<'a> {
	left: Expression<'a>,
	operator: Token<'a>,
	right: Expression<'a>,
}

impl<'a> Node<'a> for Operation<'a> {
	fn children(&self) -> Box<[&'a dyn Node]> {
		return Box::new([&self.left, &self.operator, &self.right]);
	}
}
