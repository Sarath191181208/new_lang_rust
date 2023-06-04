pub mod lexer;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use lexer::{Lexer, Token, TokenKind};

    struct SingleTokenTestCase {
        kind: TokenKind,
        literal: &'static str,
        start: usize,
        end: usize,
    }

    fn check_token(token: Option<Token>, kind: TokenKind, literal: &str, start: usize, end: usize) {
        assert_eq!(token.is_some(), true);
        let token = token.unwrap();
        assert_eq!(token.kind, kind, "Token kind should be equal to the expected kind");
        assert_eq!(token.span.literal, literal, "Token literal should be equal to the expected literal");
        assert_eq!(token.span.start, start, "Token start should be equal to the expected start");
        assert_eq!(token.span.end, end, "Token end should be equal to the expected end");
    }

    fn is_vector_correct(vec: &Vec<SingleTokenTestCase>, input: &str){
        let mut sum = 0;
        for test_case in vec{
            sum += test_case.end - test_case.start;
        }

        assert_eq!(sum, input.len(), "The sum of all the test cases' lengths should be equal to the input's length");
    }

    #[test]
    fn test() {
        let input = "1 + 2";
        let mut lexer = Lexer::new(input);
        let mut expected_lexed_output = vec![
            SingleTokenTestCase {
                kind: TokenKind::Number,
                literal: "1",
                start: 0,
                end: 1,
            },
            SingleTokenTestCase {
                kind: TokenKind::Whitespace,
                literal: " ",
                start: 1,
                end: 2,
            },
            SingleTokenTestCase {
                kind: TokenKind::Plus,
                literal: "+",
                start: 2,
                end: 3,
            },
            SingleTokenTestCase {
                kind: TokenKind::Whitespace,
                literal: " ",
                start: 3,
                end: 4,
            },
            SingleTokenTestCase {
                kind: TokenKind::Number,
                literal: "2",
                start: 4,
                end: 5,
            },
            SingleTokenTestCase {
                kind: TokenKind::EOF,
                literal: "\0",
                start: 0,
                end: 0,
            },
        ];

        is_vector_correct(&expected_lexed_output, &input);

        while let Some(token) = lexer.next_token() {
            let expected_lexed_output = expected_lexed_output.remove(0);
            check_token(Some(token), expected_lexed_output.kind, expected_lexed_output.literal, expected_lexed_output.start, expected_lexed_output.end);
        }
    }
}
