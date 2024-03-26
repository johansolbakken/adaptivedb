
// insert into Employee (EmployeeID, FirstName, LastName, DepartmentID, JobTitle, HireDate) values ('1', 'John', 'Doe', 1, 'Manager', '2021-01-01');
// commit;

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
        DMLLexer {
            input,
            position: 0,
        }
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
                    let token_type = match DML_KEYWORDS.iter().find(|(keyword, _)| *keyword== value) {
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
}