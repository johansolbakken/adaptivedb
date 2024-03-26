use tracing::field;

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
                position: self.position,
            });
        } else if current_char == '@' {
            token = Some(DDLToken {
                token_type: DDLTokenType::AtSign,
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '?' {
            token = Some(DDLToken {
                token_type: DDLTokenType::QuestionMark,
                position: self.position,
            });
            self.position += 1;
        } else if current_char == ',' {
            token = Some(DDLToken {
                token_type: DDLTokenType::Comma,
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '(' {
            token = Some(DDLToken {
                token_type: DDLTokenType::OpenParen,
                position: self.position,
            });
            self.position += 1;
        } else if current_char == ')' {
            token = Some(DDLToken {
                token_type: DDLTokenType::CloseParen,
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '{' {
            token = Some(DDLToken {
                token_type: DDLTokenType::OpenBrace,
                position: self.position,
            });
            self.position += 1;
        } else if current_char == '}' {
            token = Some(DDLToken {
                token_type: DDLTokenType::CloseBrace,
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

#[derive(Debug, PartialEq)]
struct Model {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
enum FieldType {
    Int,
    Float,
    String,
    Date,
    Blob,
}

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    field_type: FieldType,
    is_nullable: bool,
    is_primary_key: bool,
    is_foreign_key: bool,
    references: Option<(String, String)>,
}

struct DDLParser {
    tokens: Vec<DDLToken>,
    position: usize,
}

impl DDLParser {
    fn new(tokens: Vec<DDLToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse_model(&mut self) -> Option<Model> {
        if self.position >= self.tokens.len() {
            return None;
        }
        let mut model = Model {
            name: String::new(),
            fields: Vec::new(),
        };

        let mut token = &self.tokens[self.position];
        if token.token_type != DDLTokenType::Model {
            return None;
        }
        self.position += 1;

        token = &self.tokens[self.position];
        match &token.token_type {
            DDLTokenType::Identifier(name) => {
                model.name = name.clone();
            }
            _ => return None,
        }
        self.position += 1;

        token = &self.tokens[self.position];
        if token.token_type != DDLTokenType::OpenBrace {
            return None;
        }
        self.position += 1;

        while self.position < self.tokens.len() {
            let field = self.parse_field();
            if let Some(field) = field {
                model.fields.push(field);
            } else {
                break;
            }
        }

        Some(model)
    }

    fn parse_field(&mut self) -> Option<Field> {
        if self.position >= self.tokens.len() {
            return None;
        }
        let mut field = Field {
            name: String::new(),
            field_type: FieldType::Int,
            is_nullable: false,
            is_primary_key: false,
            is_foreign_key: false,
            references: None,
        };

        let mut token = &self.tokens[self.position];
        match &token.token_type {
            DDLTokenType::Identifier(name) => {
                field.name = name.clone();
            }
            _ => return None,
        }
        self.position += 1;

        token = &self.tokens[self.position];
        match &token.token_type {
            DDLTokenType::Type(t) => {
                field.field_type = match &t[..] {
                    "Int" => FieldType::Int,
                    "Float" => FieldType::Float,
                    "String" => FieldType::String,
                    "Date" => FieldType::Date,
                    _ => FieldType::Int,
                };
            }
            _ => return None,
        }
        self.position += 1;

        token = &self.tokens[self.position];
        if token.token_type == DDLTokenType::QuestionMark {
            field.is_nullable = true;
            self.position += 1;
        }

        token = &self.tokens[self.position];
        while token.token_type == DDLTokenType::AtSign {
            self.position += 1;
            token = &self.tokens[self.position];
            if token.token_type == DDLTokenType::Identifier("id".to_string()) {
                field.is_primary_key = true;
                self.position += 1;
            } else if token.token_type == DDLTokenType::Identifier("references".to_string()) {
                self.position += 1;
                token = &self.tokens[self.position];

                if !matches!(token.token_type, DDLTokenType::OpenParen) {
                    return None;
                }

                let model = match &self.tokens[self.position + 1].token_type {
                    DDLTokenType::Identifier(id) => id.clone(),
                    _ => return None,
                };

                let model_field = match &self.tokens[self.position + 2].token_type {
                    DDLTokenType::Identifier(id) => id.clone(),
                    _ => return None,
                };

                field.references = Some((model, model_field));
                self.position += 3;

                token = &self.tokens[self.position];
                if !matches!(token.token_type, DDLTokenType::CloseParen) {
                    return None;
                }

                self.position += 1;
            }
            token = &self.tokens[self.position];
        }

        Some(field)
    }
}

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

    #[test]
    fn test_ddl_parser() {
        let ddl = "model Employee { EmployeeID String @id FirstName String LastName String DepartmentID Int? JobTitle String? HireDate Date }";
        let correct_model = vec![Model {
            name: "Employee".to_string(),
            fields: vec![
                Field {
                    name: "EmployeeID".to_string(),
                    field_type: FieldType::String,
                    is_nullable: false,
                    is_primary_key: true,
                    is_foreign_key: false,
                    references: None,
                },
                Field {
                    name: "FirstName".to_string(),
                    field_type: FieldType::String,
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
                Field {
                    name: "LastName".to_string(),
                    field_type: FieldType::String,
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
                Field {
                    name: "DepartmentID".to_string(),
                    field_type: FieldType::Int,
                    is_nullable: true,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
                Field {
                    name: "JobTitle".to_string(),
                    field_type: FieldType::String,
                    is_nullable: true,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
                Field {
                    name: "HireDate".to_string(),
                    field_type: FieldType::Date,
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
            ],
        }];
        let mut lexer = DDLLexer::new(ddl.to_string());
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        let mut parser = DDLParser::new(tokens);
        let mut models = Vec::new();
        while let Some(model) = parser.parse_model() {
            models.push(model);
        }
        assert_eq!(models, correct_model);
    }

    // #[test]
    // fn test_ddl_correct() {
    //     let lexer = DDLLexer {};
    //     let parser = DDLParser {};
    //     let analyzer = DDLAnalyzer {};
    // }
}
