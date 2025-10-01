#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub position: usize
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    StringNotTerminated
}
impl ErrorKind {
    fn as_str(&self) -> &str {
        match self {
            ErrorKind::StringNotTerminated => "StringNotTerminated"
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
    pub message: String,
    pub notes: Vec<String>,
}

impl Error {
    pub fn pretty() -> String {


        todo!()
    }
}