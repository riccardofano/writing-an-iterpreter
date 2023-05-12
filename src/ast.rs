use std::fmt::Display;

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let matched = match self {
            Statement::LetStatement(ident, value) => format!("let {} = {value};", ident.0),
            Statement::ReturnStatement(value) => format!("return {value};"),
            Statement::ExpressionStatement(value) => value.to_string(),
        };
        write!(f, "{matched}")
    }
}

#[derive(Debug)]
pub enum Expression {
    Placeholder,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let matched = match self {
            Expression::Placeholder => "PLACEHOLDER",
        };
        write!(f, "{matched}")
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Program {
    pub fn token_literal(&self) -> String {
        let Some(statement) = self.statements.get(0) else {
            return String::new();
        };
        todo!()
    }
}
