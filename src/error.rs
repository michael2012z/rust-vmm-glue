#[derive(Debug)]
pub enum Error {
    Generic,
}
pub type Result<T> = std::result::Result<T, Error>;
