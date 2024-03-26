use tracing::info;


#[derive(Debug, PartialEq, Clone)]
enum DMLTokenType {
    Insert,
    Into,
    Values,
    Commit,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    Identifier(String),
    String(String),
    Number(f64),
    SemiColon,
}

const DML_KEYWORDS: [(&str, DMLTokenType); 4] = [
    ("insert", DMLTokenType::Insert),
    ("into", DMLTokenType::Into),
    ("values", DMLTokenType::Values),
    ("commit", DMLTokenType::Commit),
];

#[derive(Debug, PartialEq, Clone)]
struct DMLToken {
    token_type: DMLTokenType,
    position: usize,
}

struct DMLLexer {
    input: String,
    position: usize,
}

impl DMLLexer {
    fn new(input: String) -> DMLLexer {
        DMLLexer { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<DMLToken> {
        if self.position >= self.input.len() {
            return None;
        }

        let mut token = None;

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            match c {
                ' ' => {
                    self.position += 1;
                }
                '(' => {
                    token = Some(DMLToken {
                        token_type: DMLTokenType::OpenParenthesis,
                        position: self.position,
                    });
                    self.position += 1;
                    break;
                }
                ')' => {
                    token = Some(DMLToken {
                        token_type: DMLTokenType::CloseParenthesis,
                        position: self.position,
                    });
                    self.position += 1;
                    break;
                }
                ',' => {
                    token = Some(DMLToken {
                        token_type: DMLTokenType::Comma,
                        position: self.position,
                    });
                    self.position += 1;
                    break;
                }
                ';' => {
                    token = Some(DMLToken {
                        token_type: DMLTokenType::SemiColon,
                        position: self.position,
                    });
                    self.position += 1;
                    break;
                }
                '\'' => {
                    let mut value = String::new();
                    self.position += 1;
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if c == '\'' {
                            break;
                        }
                        value.push(c);
                        self.position += 1;
                    }
                    token = Some(DMLToken {
                        token_type: DMLTokenType::String(value),
                        position: self.position,
                    });
                    self.position += 1;
                    break;
                }
                '0'..='9' => {
                    let mut value = String::new();
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if c == ' ' || c == '(' || c == ')' || c == ',' || c == ';' {
                            break;
                        }
                        value.push(c);
                        self.position += 1;
                    }
                    token = Some(DMLToken {
                        token_type: DMLTokenType::Number(value.parse().unwrap()),
                        position: self.position,
                    });
                    break;
                }
                _ => {
                    let mut value = String::new();
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if c == ' ' || c == '(' || c == ')' || c == ',' || c == ';' {
                            break;
                        }
                        value.push(c);
                        self.position += 1;
                    }
                    let token_type =
                        match DML_KEYWORDS.iter().find(|(keyword, _)| *keyword == value) {
                            Some((_, token_type)) => (*token_type).clone(),
                            None => DMLTokenType::Identifier(value),
                        };
                    token = Some(DMLToken {
                        token_type,
                        position: self.position,
                    });
                    break;
                }
            }
        }

        token
    }
}

enum DMLStatement {
    Insert(DMLInsertStatement),
    Update,
    Delete,
    Select,
    Commit,
}

struct DMLInsertStatement {
    table_name: String,
    columns: Vec<String>,
    values: Vec<String>,
}

struct DMLParser {
    lexer: DMLLexer,
    current_token: Option<DMLToken>,
    peek_token: Option<DMLToken>,
}

impl DMLParser {
    fn new(input: String) -> DMLParser {
        let mut lexer = DMLLexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        DMLParser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Option<DMLStatement> {
        match self.current_token.clone() {
            Some(token) => {
                match token.token_type {
                    DMLTokenType::Insert => match self.parse_insert_statement() {
                        Some(insert_statement) => {
                            return Some(DMLStatement::Insert(insert_statement));
                        }
                        None => {}
                    },
                    DMLTokenType::Commit => {
                        self.next_token();
                        if self.current_token.clone().unwrap().token_type == DMLTokenType::SemiColon
                        {
                            self.next_token();
                            return Some(DMLStatement::Commit);
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }

        None
    }

    fn parse_insert_statement(&mut self) -> Option<DMLInsertStatement> {
        let mut table_name = String::new();
        let mut columns = Vec::new();
        let mut values = Vec::new();

        // insert
        self.next_token();

        // into
        if self.current_token.clone().unwrap().token_type != DMLTokenType::Into {
            return None;
        }
        self.next_token();

        // table name
        match self.current_token.clone() {
            Some(token) => match token.token_type {
                DMLTokenType::Identifier(value) => {
                    table_name = value;
                }
                _ => {}
            },
            None => {}
        }

        // columns
        self.next_token();

        match self.current_token.clone() {
            Some(token) => match token.token_type {
                DMLTokenType::OpenParenthesis => {
                    self.next_token();
                    while let Some(token) = self.current_token.clone() {
                        match token.token_type {
                            DMLTokenType::Identifier(value) => {
                                columns.push(value);
                            }
                            DMLTokenType::Comma => {}
                            DMLTokenType::CloseParenthesis => {
                                break;
                            }
                            _ => {}
                        }
                        self.next_token();
                    }
                }
                _ => {}
            },
            None => {}
        }

        // values
        self.next_token();

        match self.current_token.clone() {
            Some(token) => match token.token_type {
                DMLTokenType::Values => {
                    self.next_token();
                    match self.current_token.clone() {
                        Some(token) => match token.token_type {
                            DMLTokenType::OpenParenthesis => {
                                self.next_token();
                                while let Some(token) = self.current_token.clone() {
                                    match token.token_type {
                                        DMLTokenType::String(value) => {
                                            values.push(value);
                                        }
                                        DMLTokenType::Comma => {}
                                        DMLTokenType::CloseParenthesis => {
                                            break;
                                        }
                                        _ => {}
                                    }
                                    self.next_token();
                                }
                            }
                            _ => {}
                        },
                        None => {}
                    }
                }
                _ => {}
            },
            None => {}
        }

        // )
        self.next_token();

        // ;
        if self.current_token.clone().unwrap().token_type != DMLTokenType::SemiColon {
            return None;
        }

        self.next_token();

        Some(DMLInsertStatement {
            table_name,
            columns,
            values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DMLTokenType;

    #[test]
    fn test_dml_lexer() {
        let input = "insert into Employee (EmployeeID, FirstName, LastName) values ('1', 'John', 'Doe'); commit;";
        let correct_sequence = vec![
            DMLTokenType::Insert,
            DMLTokenType::Into,
            DMLTokenType::Identifier("Employee".to_string()),
            DMLTokenType::OpenParenthesis,
            DMLTokenType::Identifier("EmployeeID".to_string()),
            DMLTokenType::Comma,
            DMLTokenType::Identifier("FirstName".to_string()),
            DMLTokenType::Comma,
            DMLTokenType::Identifier("LastName".to_string()),
            DMLTokenType::CloseParenthesis,
            DMLTokenType::Values,
            DMLTokenType::OpenParenthesis,
            DMLTokenType::String("1".to_string()),
            DMLTokenType::Comma,
            DMLTokenType::String("John".to_string()),
            DMLTokenType::Comma,
            DMLTokenType::String("Doe".to_string()),
            DMLTokenType::CloseParenthesis,
            DMLTokenType::SemiColon,
            DMLTokenType::Commit,
            DMLTokenType::SemiColon,
        ];

        let mut lexer = super::DMLLexer::new(input.to_string());
        for token in correct_sequence.iter() {
            let next_token = lexer.next_token().unwrap();
            assert_eq!(next_token.token_type, *token);
        }
    }

    #[test]
    fn test_dml_parser_insert_statement() {
        let input = "insert into Employee (EmployeeID, FirstName, LastName) values ('1', 'John', 'Doe'); commit;";
        let mut parser = super::DMLParser::new(input.to_string());
        let statement = parser.parse();
        match statement {
            Some(super::DMLStatement::Insert(insert_statement)) => {
                assert_eq!(insert_statement.table_name, "Employee");
                assert_eq!(
                    insert_statement.columns,
                    vec!["EmployeeID", "FirstName", "LastName"]
                );
                assert_eq!(insert_statement.values, vec!["1", "John", "Doe"]);
            }
            _ => {
                assert!(false);
            }
        }

        let statement = parser.parse();
        match statement {
            Some(super::DMLStatement::Commit) => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }
}
