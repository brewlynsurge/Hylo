#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    SyntaxError,
    StringNotTerminated
}

impl ErrorKind {
    pub fn code(&self) -> &'static str {
        match self {
            ErrorKind::SyntaxError => "E0001",
            ErrorKind::StringNotTerminated=> "E0002",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::SyntaxError => "SyntaxError",
            ErrorKind::StringNotTerminated => "StringNotTerminated",

        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            ErrorKind::SyntaxError => 1,
            ErrorKind::StringNotTerminated => 1,

        }
    }
}