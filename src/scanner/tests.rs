use super::{token::TokenType, Scanner, Token};

#[cfg(test)]
mod tests {
    use super::*;

    fn get_token_types(source: &str) -> Vec<TokenType> {
        let mut scanner = Scanner::new(source.into());
        scanner.tokenize().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_general_cases() {
        assert_eq!(
            get_token_types("1 + 5 * (1 + 9);"),
            vec![
                TokenType::Number(1.0 as f64),
                TokenType::Plus,
                TokenType::Number(5.0 as f64),
                TokenType::Star,
                TokenType::LeftParen,
                TokenType::Number(1.0 as f64),
                TokenType::Plus,
                TokenType::Number(9.0 as f64),
                TokenType::RightParen,
                TokenType::Semicolon,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_single_char_tokens() {
        assert_eq!(
            get_token_types("( ) + - * / = ;"),
            vec![
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Star,
                TokenType::Slash,
                TokenType::Assign,
                TokenType::Semicolon,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(
            get_token_types("== != === !== > < >= <="),
            vec![
                TokenType::Equal,
                TokenType::NotEqual,
                TokenType::StrictEqual,
                TokenType::StrictNotEqual,
                TokenType::GreaterThan,
                TokenType::LessThan,
                TokenType::GreaterThanOrEqual,
                TokenType::LessThanOrEqual,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_number_literals() {
        assert_eq!(
            get_token_types("123 12.3 0 0.0 .123 123."),
            vec![
                TokenType::Number(123 as f64),
                TokenType::Number(12.3 as f64),
                TokenType::Number(0 as f64),
                TokenType::Number(0.0 as f64),
                TokenType::Number(0.123 as f64),
                TokenType::Number(123 as f64),
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_literals() {
        assert_eq!(
            get_token_types(r#"123 false 'hello' true null undefined "hello" `hello`"#),
            vec![
                TokenType::Number(123 as f64),
                TokenType::Boolean(false),
                TokenType::String("hello".into()),
                TokenType::Boolean(true),
                TokenType::Null,
                TokenType::Undefined,
                TokenType::String("hello".into()),
                TokenType::String("hello".into()),
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_identifiers_and_keywords() {
        assert_eq!(
            get_token_types("let const hello _hello x0"),
            vec![
                TokenType::KeywordLet,
                TokenType::KeywordConst,
                TokenType::Identifier("hello".into()),
                TokenType::Identifier("_hello".into()),
                TokenType::Identifier("x0".into()),
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(
            get_token_types("// Line comment \nlet x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Assign,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_block_comment() {
        assert_eq!(
            get_token_types("let /* block comment */ x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Assign,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_nested_block_comment() {
        assert_eq!(
            get_token_types("let /* block /* nested */ comment */ x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Assign,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_multi_line_nested_block_comment() {
        assert_eq!(
            get_token_types("let /* block \n * \n * /* nested */ \n * \n * comment */ x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Assign,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(get_token_types(""), vec![TokenType::Eof]);

        assert_eq!(get_token_types("   \t\n\r"), vec![TokenType::Eof]);
    }
}
