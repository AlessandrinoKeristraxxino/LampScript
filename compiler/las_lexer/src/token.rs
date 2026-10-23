// lampscript_compiler/compiler/las_lexer/src/token.rs

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    Fn,
    If,
    Else,
    Let,
    Return,
    Ident,
    Number,
    String,
    Mod,

    OpenAngle,
    CloseAngle,
    Slash,
    Colon,
    Comma,

    OpenBrace,
    CloseBrace,
    Eq,
    Whitespace,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
