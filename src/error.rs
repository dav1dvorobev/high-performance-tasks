//! Contains [Error] and corresponding [Result].

/// A result with a specified [Error] type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Represents all possible errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    MPIError(#[from] mpi::MpiError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
