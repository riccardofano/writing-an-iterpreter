use std::fmt::Display;

#[derive(Debug)]
pub enum ObjectKind {
    Integer(usize),
    Boolean(bool),
    Null,
}

impl ObjectKind {
    pub fn inspect(&self) -> String {
        match self {
            ObjectKind::Integer(int) => int.to_string(),
            ObjectKind::Boolean(bool) => bool.to_string(),
            ObjectKind::Null => "null".to_string(),
        }
    }
}

impl Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            ObjectKind::Integer(_) => "INTEGER",
            ObjectKind::Boolean(_) => "BOOLEAN",
            ObjectKind::Null => "NULL",
        };
        write!(f, "{kind}")
    }
}

#[derive(Debug)]
pub struct Object {
    pub kind: ObjectKind,
}

impl Object {
    pub fn new(kind: ObjectKind) -> Self {
        Self { kind }
    }
}
