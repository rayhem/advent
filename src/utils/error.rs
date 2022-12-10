#[derive(Debug)]
pub enum Error {
    Unimplemented,
    ParseError,
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
