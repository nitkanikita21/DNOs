// #[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("There is no color for this byte ({0})")]
    InvalidColor(u8),
}