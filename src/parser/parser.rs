use crate::lexer::token::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}
