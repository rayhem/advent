#[derive(Clone, Debug)]
pub enum Error {
    InvalidFile(std::path::PathBuf),
    ParseError,
    Unimplemented,
}
