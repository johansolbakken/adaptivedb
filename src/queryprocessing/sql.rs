#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Select,
    From,
    Where,
    Insert,
    Update,
    Delete,
    Create,
    Drop,
    Alter,
    Join,
    Inner,
    Left,
    Right,
    Full,
    On,
    Into,
    Values,
    Set,
    And,
    Or,
    Not,
    Null,
    As,
    Distinct,
    Order,
    By,
    Group,
    Having,
    Limit,
    Offset,
    Union,
    All,
    In,
    Is,
    Like,
    Exists,
    Between,
    Case,
    When,
    Then,
    Else,
    End,
    // New Keywords
    Primary,
    Key,
    Foreign,
    Check,
    Default,
    Begin,
    Commit,
    Rollback,
    Savepoint,
    Truncate,
    Replace,
    Merge,
    Lock,
    Grant,
    Revoke,
    Intersect,
    Except,
    Fetch,
    Over,
    Partition,

    // Data Types
    Int,
    Varchar,
    Char,
    Date,
    Timestamp,
    Float,
    Double,
    Boolean,
    // New Data Types
    Time,
    Blob,
    Clob,
    Text,
    Decimal,
    Numeric,
    SmallInt,
    BigInt,
    Real,
    Interval,

    // Operators
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // New Operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Concat, // ||

    // Symbols
    Comma,
    Dot,
    Semicolon,
    OpenParen,
    CloseParen,
    // New Symbols
    OpenBracket,  // [
    CloseBracket, // ]
    Dollar,       // $

    // Literals
    Identifier(String),    // For column names, table names, etc.
    StringLiteral(String), // For '...' strings
    NumericLiteral(f64),   // For numeric constants like 123 or 45.67
    // Escaped Identifier
    QuotedIdentifier(String), // "column_name" or "table_name"

    // Comments
    Comment(String), // For -- or /* */

    // End of Input
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: String,
    position: usize, // Current position in input
    line: usize,     // Current line for error reporting
    column: usize,   // Current column for error reporting
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position].to_string()
    }

    fn read_number(&mut self) -> f64 {
        let start_pos = self.position;
        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position]
            .parse::<f64>()
            .unwrap_or(0.0)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token_type = match self.peek()? {
            '=' => {
                self.advance();
                TokenType::Equals
            }
            '<' => {
                self.advance();
                match self.peek()? {
                    '=' => {
                        self.advance();
                        TokenType::LessThanOrEquals
                    }
                    '>' => {
                        self.advance();
                        TokenType::NotEquals
                    }
                    _ => TokenType::LessThan,
                }
            }
            '>' => {
                self.advance();
                if self.peek()? == '=' {
                    self.advance();
                    TokenType::GreaterThanOrEquals
                } else {
                    TokenType::GreaterThan
                }
            }
            ',' => {
                self.advance();
                TokenType::Comma
            }
            '.' => {
                self.advance();
                TokenType::Dot
            }
            ';' => {
                self.advance();
                TokenType::Semicolon
            }
            '(' => {
                self.advance();
                TokenType::OpenParen
            }
            ')' => {
                self.advance();
                TokenType::CloseParen
            }
            '+' => {
                self.advance();
                TokenType::Plus
            }
            '-' => {
                self.advance();
                if self.peek()? == '-' {
                    // -- is comment
                    while !(self.peek()? == '\n') {
                        self.advance();
                    }
                    self.advance();
                    self.next_token()?.token_type
                } else {
                    TokenType::Minus
                }
            }
            '*' => {
                self.advance();
                TokenType::Multiply
            }
            '/' => {
                self.advance();
                TokenType::Divide
            }
            '0'..='9' => {
                let number = self.read_number();
                TokenType::NumericLiteral(number)
            }
            c if c.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.to_uppercase().as_str() {
                    "SELECT" => TokenType::Select,
                    "FROM" => TokenType::From,
                    "WHERE" => TokenType::Where,
                    _ => TokenType::Identifier(identifier),
                }
            }
            _ => return None, // Unrecognized token
        };

        Some(Token {
            token_type,
            line: self.line,
            column: self.column,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_token() {
        let input = String::from("SELECT");
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Select);
    }

    #[test]
    fn test_basic_tokens() {
        let input = String::from("SELECT * FROM users WHERE age = 30;");
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Select);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Multiply);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::From);

        let token = lexer.next_token().unwrap();
        assert_eq!(
            token.token_type,
            TokenType::Identifier(String::from("users"))
        );

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Where);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier(String::from("age")));

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Equals);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NumericLiteral(30.0));

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Semicolon);

        let token = lexer.next_token();
        assert!(token.is_none()); // Should be end of input
    }

    #[test]
    fn test_identifier_token() {
        let input = String::from("users");
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token().unwrap();
        assert_eq!(
            token.token_type,
            TokenType::Identifier(String::from("users"))
        );
    }

    #[test]
    fn test_numeric_literal() {
        let input = String::from("123");
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NumericLiteral(123.0));
    }

    #[test]
    fn test_operator_tokens() {
        let input = String::from("= <> < > <= >= + - * / %");
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Equals);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NotEquals);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::LessThan);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::GreaterThan);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::LessThanOrEquals);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::GreaterThanOrEquals);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Plus);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Minus);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Multiply);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Divide);
    }

    #[test]
    fn test_parentheses_and_commas() {
        let input = String::from("( ) , .");
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::OpenParen);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::CloseParen);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Comma);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Dot);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = String::from("SELECT    *  FROM \n users \t WHERE age = 30;");
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Select);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Multiply);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::From);

        let token = lexer.next_token().unwrap();
        assert_eq!(
            token.token_type,
            TokenType::Identifier(String::from("users"))
        );

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Where);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier(String::from("age")));

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Equals);

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NumericLiteral(30.0));

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Semicolon);
    }

    #[test]
    fn test_comment_handling() {
        let input = String::from("-- This is a comment\nSELECT * FROM users;");
        let mut lexer = Lexer::new(input);

        // We could add support for comments in the lexer, but for now, let's skip it
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Select);
    }
}
