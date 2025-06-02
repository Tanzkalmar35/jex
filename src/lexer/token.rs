use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    #[regex(r"//[^\n\r]*", logos::skip)]
    LineComment,

    #[token(":")]
    Separator,

    #[regex(r#""([^"\\]|\\.)*""#)]
    ValueString,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*")]
    Identifier,

    #[regex(r"[0-9]+")]
    ValueInt,
}
