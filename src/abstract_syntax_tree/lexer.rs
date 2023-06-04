#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
    BadToken,
    Unimplemented,
    EOF, // End of File
}

impl std::fmt::Display for TokenKind{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

#[derive(Debug, PartialEq)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.is_file_ended() {
            return None;
        }

        if self.is_last_char_of_file() {
            self.current_pos += 1;
            let span = TextSpan::new(0, 0, "\0".to_owned());
            return Some(Token::new(TokenKind::EOF, span));
        }

        let start_idx = self.current_pos;
        let current_char = self.get_current_indexed_char();
        let kind: TokenKind;

        if Self::is_number(&current_char) {
            let _number = self.parse_number();
            kind = TokenKind::Number;
        } else {
            kind = self.parse_arthmatic_symbols();
        }

        let end_index = self.current_pos;
        let literal = self.input[start_idx..end_index].to_string();
        let span = TextSpan::new(start_idx, end_index, literal);
        let token = Token::new(kind, span);
        Some(token)
    }

    fn is_number(_char: &char) -> bool {
        _char.is_digit(10)
    }

    fn parse_number(&mut self) -> i64 {
        let mut number: i64 = 0; // TODO: number out of bounds
        while let Some(_char) = self.consume_character() {
            if Self::is_number(&_char) {
                let digit = _char.to_digit(10).unwrap() as i64;
                number = number * 10 + digit;
            } else {
                self.current_pos -= 1;
                break;
            }
        }

        number
    }

    fn parse_arthmatic_symbols(&mut self) -> TokenKind {
        match self.consume_character() {
            None => TokenKind::BadToken,
            // = => 
            Some(c) => match c {
                '+' => TokenKind::Plus, 
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '(' => TokenKind::OpenParenthesis,
                ')' => TokenKind::CloseParenthesis,
                ' ' => TokenKind::Whitespace,
                _ => TokenKind::Unimplemented,
            },
        }
    }

    fn is_file_ended(&self) -> bool {
        self.current_pos > self.input.len()
    }

    fn is_last_char_of_file(&self) -> bool {
        self.current_pos == self.input.len()
    }

    fn get_current_indexed_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }

    fn consume_character(&mut self) -> Option<char> {
        if self.is_file_ended() || self.is_last_char_of_file() {
            return None;
        }
        let c = self.get_current_indexed_char();
        self.current_pos += 1;
        Some(c)
    }
}
