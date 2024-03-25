use std::thread::current;

#[derive(Debug, PartialEq)]
enum DDLTokenType {
    Model,
    QuestionMark,
    AtSign,
    Type(String),
    Identifier(String),
    OpenBrace,
    CloseBrace,
    Comma,
    OpenParen,
    CloseParen,
}

struct DDLToken {
    token_type: DDLTokenType,
    value: String,
    position: usize,
}

struct DDLLexer {
    input: String,
    position: usize,
    errors: Vec<String>,
}

impl DDLLexer {
    fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) -> Option<DDLToken> {
        if self.position >= self.input.len() {
            return None;
        }
        let mut token = None;
        let mut current_char = self.input.chars().nth(self.position).unwrap();
        if current_char.is_whitespace() {
            self.position += 1;
            return self.next_token();
        }

        if current_char.is_alphabetic() {
            let mut value = String::new();
            while current_char.is_alphabetic() {
                value.push(current_char);
                self.position += 1;
                current_char = self.input.chars().nth(self.position).unwrap();
            }

            let types = vec!["Int", "Float", "String", "Date"];
            let token_type = if types.contains(&&value[..]) {
                DDLTokenType::Type(value.clone())
            } else if value == "model" {
                DDLTokenType::Model
            } else {
                DDLTokenType::Identifier(value.clone())
            };

            token = Some(DDLToken {
                token_type,
                value,
                position: self.position,
            });
        } else if current_char == '@' {
            token = Some(DDLToken {
                token_type: DDLTokenType::AtSign,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '?' {
            token = Some(DDLToken {
                token_type: DDLTokenType::QuestionMark,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else if current_char == ',' {
            token = Some(DDLToken {
                token_type: DDLTokenType::Comma,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '(' {
            token = Some(DDLToken {
                token_type: DDLTokenType::OpenParen,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else if current_char == ')' {
            token = Some(DDLToken {
                token_type: DDLTokenType::CloseParen,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '{' {
            token = Some(DDLToken {
                token_type: DDLTokenType::OpenBrace,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '}' {
            token = Some(DDLToken {
                token_type: DDLTokenType::CloseBrace,
                value: current_char.to_string(),
                position: self.position,
            });
            self.position += 1;
        } else {
            self.errors
                .push(format!("Invalid character: {}", current_char));
            self.position += 1;
        }

        token
    }
}

struct DDLParser {}

struct DDLAnalyzer {}

#[cfg(test)]
mod tests {
    use super::*;

    const DDL_CORRECT: &str = "
        model Employee {
            EmployeeID String @id
            FirstName String
            LastName String 
            DepartmentID Int?
            JobTitle String? 
            HireDate Date 
        }

        model Department {
            DepartmentID Int @id
            DepartmentName String
        }

        model Salary {
            SalaryID Int
            EmployeeID Int @references(Employee, EmployeeID)
            Salary Float
            FromDate Date
            ToDate Date? 
        }";

    const DDL_MISSING_PRIMARY_KEY: &str = "
        model Employee {
            EmployeeID String
        }
        ";

    const DDL_MISSING_FOREIGN_KEY_MODEL: &str = "
        model Salary {
            SalaryID Int @id
            EmployeeID Int @references(Employee, EmployeeID)
            Salary Float
            FromDate Date
            ToDate Date? 
        }";

    #[test]
    fn test_ddl_lexer() {
        let ddl = "model {}() identifier Int Float Date ? , @";
        let correct_sequence = vec![
            DDLTokenType::Model,
            DDLTokenType::OpenBrace,
            DDLTokenType::CloseBrace,
            DDLTokenType::OpenParen,
            DDLTokenType::CloseParen,
            DDLTokenType::Identifier("identifier".to_string()),
            DDLTokenType::Type("Int".to_string()),
            DDLTokenType::Type("Float".to_string()),
            DDLTokenType::Type("Date".to_string()),
            DDLTokenType::QuestionMark,
            DDLTokenType::Comma,
            DDLTokenType::AtSign,
        ];
        let mut lexer = DDLLexer::new(ddl.to_string());
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token.token_type);
        }
        assert_eq!(tokens, correct_sequence);
    }

    // #[test]
    // fn test_ddl_correct() {
    //     let lexer = DDLLexer {};
    //     let parser = DDLParser {};
    //     let analyzer = DDLAnalyzer {};
    // }
}
