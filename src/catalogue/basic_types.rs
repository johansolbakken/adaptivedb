
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasicType {
    Int,
    Float,
    Bool,
    String,
    Date,
    Blob
}

impl BasicType {
    pub fn from_str(s: &str) -> Option<BasicType> {
        match s {
            "Int" => Some(BasicType::Int),
            "Float" => Some(BasicType::Float),
            "Bool" => Some(BasicType::Bool),
            "String" => Some(BasicType::String),
            "Date" => Some(BasicType::Date),
            "Blob" => Some(BasicType::Blob),
            _ => None
        }
    }
}