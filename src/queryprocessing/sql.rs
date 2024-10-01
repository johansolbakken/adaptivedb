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
    
    Table,

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
                    // Handle single-line comments starting with --
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
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
            '\'' => {
                // Handle string literals
                self.advance();
                let mut string_literal = String::new();
                while let Some(c) = self.peek() {
                    if c == '\'' {
                        self.advance();
                        break;
                    }
                    string_literal.push(c);
                    self.advance();
                }
                TokenType::StringLiteral(string_literal)
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
                    "INSERT" => TokenType::Insert,
                    "INTO" => TokenType::Into,
                    "VALUES" => TokenType::Values,
                    "CREATE" => TokenType::Create,
                    "TABLE" => TokenType::Table,
                    "DROP" => TokenType::Drop,
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

#[derive(Debug, PartialEq, Clone)]
pub enum SQLQuery {
    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<String>,
    },
    CreateTable {
        table_name: String,
        columns: Vec<String>,
    },
    DropTable {
        table_name: String,
    },
    Select {
        table_name: String,
        where_clause: Option<(String, String)>, // e.g., ("col", "val")
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    fn expect_token(&mut self, expected: TokenType) -> Result<Token, String> {
        let token = self.advance();
        match token {
            Some(token) if token.token_type == expected => Ok(token.clone()),
            Some(token) => Err(format!(
                "Unexpected token: expected {:?}, but got {:?}",
                expected, token.token_type
            )),
            None => Err(format!("Expected {:?}, but reached end of input", expected)),
        }
    }

    // Parse an identifier like table or column name
    fn parse_identifier(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => Ok(name.clone()),
            _ => Err("Expected identifier".to_string()),
        }
    }

    // Parse an SQL value (basic for now, treating them as strings)
    fn parse_value(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::StringLiteral(value),
                ..
            }) => Ok(value.clone()),
            Some(Token {
                token_type: TokenType::NumericLiteral(value),
                ..
            }) => Ok(value.to_string()),
            _ => Err("Expected value".to_string()),
        }
    }

    // Parse the `INSERT INTO` statement
    fn parse_insert(&mut self) -> Result<SQLQuery, String> {
        self.expect_token(TokenType::Insert)?;
        self.expect_token(TokenType::Into)?;
        let table_name = self.parse_identifier()?;

        // Parse columns
        self.expect_token(TokenType::OpenParen)?;
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_identifier()?);
            if let Some(Token {
                token_type: TokenType::Comma,
                ..
            }) = self.peek()
            {
                self.advance(); // Consume comma
            } else {
                break;
            }
        }
        self.expect_token(TokenType::CloseParen)?;

        // Parse values
        self.expect_token(TokenType::Values)?;
        self.expect_token(TokenType::OpenParen)?;
        let mut values = Vec::new();
        loop {
            values.push(self.parse_value()?);
            if let Some(Token {
                token_type: TokenType::Comma,
                ..
            }) = self.peek()
            {
                self.advance(); // Consume comma
            } else {
                break;
            }
        }
        self.expect_token(TokenType::CloseParen)?;

        Ok(SQLQuery::Insert {
            table_name,
            columns,
            values,
        })
    }

    // Parse the `CREATE TABLE` statement
    fn parse_create_table(&mut self) -> Result<SQLQuery, String> {
        self.expect_token(TokenType::Create)?;
        self.expect_token(TokenType::Table)?;
        let table_name = self.parse_identifier()?;

        // For now, we’ll keep column definitions simple
        self.expect_token(TokenType::OpenParen)?;
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_identifier()?);
            if let Some(Token {
                token_type: TokenType::Comma,
                ..
            }) = self.peek()
            {
                self.advance(); // Consume comma
            } else {
                break;
            }
        }
        self.expect_token(TokenType::CloseParen)?;

        Ok(SQLQuery::CreateTable {
            table_name,
            columns,
        })
    }

    // Parse the `DROP TABLE` statement
    fn parse_drop_table(&mut self) -> Result<SQLQuery, String> {
        self.expect_token(TokenType::Drop)?;
        self.expect_token(TokenType::Table)?;
        let table_name = self.parse_identifier()?;
        Ok(SQLQuery::DropTable { table_name })
    }

    // Parse the `SELECT * FROM` statement with `WHERE` clause
    fn parse_select(&mut self) -> Result<SQLQuery, String> {
        self.expect_token(TokenType::Select)?;
        self.expect_token(TokenType::Multiply)?;
        self.expect_token(TokenType::From)?;
        let table_name = self.parse_identifier()?;

        let where_clause = if let Some(Token {
            token_type: TokenType::Where,
            ..
        }) = self.peek()
        {
            self.advance(); // Consume WHERE
            let column = self.parse_identifier()?;
            self.expect_token(TokenType::Equals)?;
            let value = self.parse_value()?;
            Some((column, value))
        } else {
            None
        };

        Ok(SQLQuery::Select {
            table_name,
            where_clause,
        })
    }

    // Main entry point for parsing
    pub fn parse_query(&mut self) -> Result<SQLQuery, String> {
        match self.peek() {
            Some(Token {
                token_type: TokenType::Insert,
                ..
            }) => self.parse_insert(),
            Some(Token {
                token_type: TokenType::Create,
                ..
            }) => self.parse_create_table(),
            Some(Token {
                token_type: TokenType::Drop,
                ..
            }) => self.parse_drop_table(),
            Some(Token {
                token_type: TokenType::Select,
                ..
            }) => self.parse_select(),
            _ => Err("Unknown SQL statement".to_string()),
        }
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

    fn tokenize(sql: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(sql.to_string());
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_parse_insert() {
        let sql = "INSERT INTO users (name, age) VALUES ('John', 30)";
        let tokens = tokenize(sql);
        let mut parser = Parser::new(tokens);

        let query = parser.parse_query().unwrap();
        assert_eq!(
            query,
            SQLQuery::Insert {
                table_name: "users".to_string(),
                columns: vec!["name".to_string(), "age".to_string()],
                values: vec!["John".to_string(), "30".to_string()],
            }
        );
    }

    #[test]
    fn test_parse_create_table() {
        let sql = "CREATE TABLE users (id, name)";
        let tokens = tokenize(sql);
        let mut parser = Parser::new(tokens);

        let query = parser.parse_query().unwrap();
        assert_eq!(
            query,
            SQLQuery::CreateTable {
                table_name: "users".to_string(),
                columns: vec!["id".to_string(), "name".to_string()],
            }
        );
    }

    #[test]
    fn test_parse_drop_table() {
        let sql = "DROP TABLE users";
        let tokens = tokenize(sql);
        let mut parser = Parser::new(tokens);

        let query = parser.parse_query().unwrap();
        assert_eq!(
            query,
            SQLQuery::DropTable {
                table_name: "users".to_string()
            }
        );
    }

    #[test]
    fn test_parse_select() {
        let sql = "SELECT * FROM users WHERE age = 30";
        let tokens = tokenize(sql);
        let mut parser = Parser::new(tokens);

        let query = parser.parse_query().unwrap();
        assert_eq!(
            query,
            SQLQuery::Select {
                table_name: "users".to_string(),
                where_clause: Some(("age".to_string(), "30".to_string())),
            }
        );
    }
}
