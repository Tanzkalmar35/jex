use logos::Logos;

use super::token::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TokenWithSpan {
    pub(crate) token: Token,
    pub(crate) span: std::ops::Range<usize>,
    pub(crate) text: String,
}

pub struct Lexer;

#[allow(dead_code)]
impl Lexer {
    /// Tokenize input and return all tokens with their spans
    pub fn tokenize(input: &str) -> Result<Vec<TokenWithSpan>, LexError> {
        let mut lexer = Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(result) = lexer.next() {
            match result {
                Ok(token) => {
                    let span = lexer.span();
                    let text = input[span.clone()].to_string();
                    tokens.push(TokenWithSpan { token, span, text });
                }
                Err(_) => {
                    let span = lexer.span();
                    let text = input[span.clone()].to_string();
                    return Err(LexError {
                        message: format!("Unexpected character: '{}'", text),
                        span,
                    });
                }
            }
        }

        Ok(tokens)
    }

    /// Quick validation without collecting tokens (for when you only need to check validity)
    pub fn is_valid(input: &str) -> bool {
        Token::lexer(input).all(|token| token.is_ok())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LexError {
    pub(crate) message: String,
    pub(crate) span: std::ops::Range<usize>,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}..{}",
            self.message, self.span.start, self.span.end
        )
    }
}

impl std::error::Error for LexError {}
