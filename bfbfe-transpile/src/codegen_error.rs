#![allow(clippy::std_instead_of_core)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError
{
    #[error("invalid backend selected")]
    InvalidBackend,
}
