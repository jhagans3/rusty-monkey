use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn node_type(&self) -> NodeType;
}

pub enum NodeType {
    Statement,
    Expression,
}

pub struct Program {
    pub statements: Vec<Box<dyn Node>>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Statement
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: dyn Node,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Statement
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Expression
    }
}
