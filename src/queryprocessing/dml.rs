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

#[derive(Debug, Clone)]
pub enum DMLStatement {
    Insert(DMLInsertStatement),
    Update,
    Delete,
    Select,
    Commit,
}

#[derive(Debug, Clone)]
pub struct DMLInsertStatement {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<String>,
}

struct DMLParser {
    lexer: DMLLexer,
    current_token: Option<DMLToken>,
    peek_token: Option<DMLToken>,
    errors: Vec<String>,
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
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Option<DMLStatement> {
        match self.current_token.clone() {
            Some(token) => match token.token_type {
                DMLTokenType::Insert => match self.parse_insert_statement() {
                    Some(insert_statement) => {
                        return Some(DMLStatement::Insert(insert_statement));
                    }
                    None => {}
                },
                DMLTokenType::Commit => {
                    self.next_token();
                    if self.current_token.clone().unwrap().token_type == DMLTokenType::SemiColon {
                        self.next_token();
                        return Some(DMLStatement::Commit);
                    }
                }
                _ => {}
            },
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

pub fn parse(query: &str) -> Option<DMLStatement> {
    let mut parser = DMLParser::new(query.to_string());
    parser.parse()
}

// Analyzer
// - Check if the table exists
// - Check if the columns exist
// - Check if the values are of the correct type
// - Check if the values are not null if the column is not nullable
// - Check if the primary key is unique
// - Check if the foreign key exists
// - Check if the foreign key is not null
// - Check if the foreign key is unique
// - Check if the foreign key is of the correct type
// - Check if the foreign key is not the primary key

struct DMLAnalyzer {
    statements: Vec<DMLStatement>,
}

impl DMLAnalyzer {
    fn new(statements: Vec<DMLStatement>) -> DMLAnalyzer {
        DMLAnalyzer { statements }
    }

    async fn analyze(&self) -> Vec<String> {
        let mut errors = Vec::new();
        errors.extend(self.check_table_exists().await);
        errors.extend(self.check_columns_exist().await);
        errors
    }

    async fn check_table_exists(&self) -> Vec<String> {
        let mut errors = Vec::new();
        for statement in self.statements.iter() {
            match statement {
                DMLStatement::Insert(insert_statement) => {
                    let catalogue = crate::get_catalogue().lock().await;
                    if !catalogue.table_exists(&insert_statement.table_name) {
                        errors.push(format!(
                            "Table {} does not exist",
                            insert_statement.table_name
                        ));
                    }
                }   
                _ => {}
            }
        }
        errors
    }

    async fn check_columns_exist(&self) -> Vec<String> {
        let mut errors = Vec::new();
        for statement in self.statements.iter() {
            match statement {
                DMLStatement::Insert(insert_statement) => {
                    let catalogue = crate::get_catalogue().lock().await;
                    let table = catalogue.get_table(&insert_statement.table_name).unwrap();
                    for column in insert_statement.columns.iter() {
                        if !table.column_exists(column) {
                            errors.push(format!(
                                "Column {} does not exist in table {}",
                                column, insert_statement.table_name
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        errors
    }

    // TODO: Implement check if values are of the correct type
    async fn check_values_correct_type(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if values are not null if the column is not nullable
    async fn check_values_not_null(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if the primary key is unique
    async fn check_primary_key_unique(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if the foreign key exists
    fn check_foreign_key_exists(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if the foreign key is not null
    fn check_foreign_key_not_null(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if the foreign key is unique
    fn check_foreign_key_unique(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if the foreign key is of the correct type
    fn check_foreign_key_correct_type(&self) -> Vec<String> {
        vec![]
    }

    // TODO: Implement check if the foreign key is not the primary key
    fn check_foreign_key_not_primary_key(&self) -> Vec<String> {
        vec![]
    }
}

pub async fn analyze(statement: DMLStatement) -> Vec<String> {
    let analyzer = DMLAnalyzer::new(vec![statement]);
    analyzer.analyze().await
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
