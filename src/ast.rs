struct Token<'a> {
	element: &'a str,
}

struct Program<'a> {
	statements: Statements<'a>,
}

struct Statements<'a> {
	statements: Vec<Statement<'a>>,
}

enum Statement<'a> {
	Expression(Box<StatementExpression<'a>>),
}

struct StatementExpression<'a> {
	expression: Expression<'a>,
	semicolon: Token<'a>
}

struct StatementStructure<'a> {
	structure: Structure<'a>,
	semicolon: Option<Token<'a>>
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

struct Expressions<'a> {
	content: Option<Box<ExpressionsContent<'a>>>,
}

struct ExpressionsContent<'a> {
	expression: Expression<'a>,
	more: Option<Box<ExpressionsMore<'a>>>,
}

struct ExpressionsMore<'a> {
	comma: Token<'a>,
	expression: Expression<'a>,
	more: Option<Box<ExpressionsMore<'a>>>,
}

struct Literal<'a> {
	literal: Token<'a>,
}

struct Group<'a> {
	open: Token<'a>,
	expression: Expression<'a>,
	close: Token<'a>,
}

struct Declaration<'a> {
	r#let: Token<'a>,
	identifier: Token<'a>,
}

struct Function<'a> {
	function: Token<'a>,
	open: Token<'a>,
	parameters: Parameters<'a>,
	close: Token<'a>,
	block: Block<'a>,
}

struct Control<'a> {
	control: Token<'a>,
	expression: Option<Box<Expression<'a>>>,
}

struct Parameters<'a> {
	content: Option<Box<ExpressionsContent<'a>>>,
}

struct ParametersContent<'a> {
	parameter: Token<'a>,
	more: Option<Box<ParametersMore<'a>>>,
}

struct ParametersMore<'a> {
	comma: Token<'a>,
	parameter: Token<'a>,
	more: Option<Box<ParametersMore<'a>>>,
}

enum Structure<'a> {
	Block(Box<Block<'a>>),
	If(Box<If<'a>>),
	Loop(Box<Loop<'a>>),
	While(Box<While<'a>>),
	For(Box<For<'a>>),
}

struct Block<'a> {
	open: Token<'a>,
	statements: Statements<'a>,
	expression: Expression<'a>,
	close: Token<'a>,
}

struct If<'a> {
	r#if: Token<'a>,
	condition: Expression<'a>,
	body: Block<'a>,
	r#else: Option<Box<Else<'a>>>,
}

struct Else<'a> {
	r#else: Token<'a>,
	body: Block<'a>,
}

struct Loop<'a> {
	r#loop: Token<'a>,
	body: Block<'a>,
}

struct While<'a> {
	r#while: Token<'a>,
	condition: Expression<'a>,
	body: Block<'a>,
}

struct For<'a> {
	r#for: Token<'a>,
	name: Token<'a>,
	r#in: Token<'a>,
	iterable: Expression<'a>,
	body: Block<'a>,
}

struct Chain<'a> {
	expression: Expression<'a>,
	dot: Token<'a>,
	identifier: Token<'a>,
}

struct Sequence<'a> {
	expression: Expression<'a>,
	open: Token<'a>,
	expressions: Expressions<'a>,
	close: Token<'a>,
}

struct Operation<'a> {
	left: Expression<'a>,
	operator: Token<'a>,
	right: Expression<'a>,
}
