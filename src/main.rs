mod abstract_syntax_tree;
use abstract_syntax_tree::lexer::{Lexer, Token, TokenKind};

use tabled::{settings::{Style, Modify, object::Rows, Alignment}, Table, Tabled};

#[derive(Tabled)]
struct TokenInfo {
    start: usize,
    end: usize,
    kind: TokenKind,
    literal: String,
}

fn main() {
    let input = "7 + 4 - 5";
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    // Assuming `tokens` is a Vec<Token> containing the token information
    let token_infos: Vec<TokenInfo> = tokens
        .iter()
        .map(|token| TokenInfo {
            start: token.span.start,
            end: token.span.end,
            kind: token.kind,
            literal: token.span.literal.to_string(),
        })
        .collect();

    let table = Table::new(&token_infos)
        .with(Style::rounded())
        .to_string()
        ;

    println!("{}", table);

    return;
}
