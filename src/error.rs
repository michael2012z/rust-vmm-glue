#[derive(Debug)]
pub enum Error {
    Generic,
    X,
}
pub type Result<T> = std::result::Result<T, Error>;
