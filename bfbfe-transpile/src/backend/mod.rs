pub mod c_backend;

use core::str::FromStr;

use bfbfe_ir::block::IRBlock;

use crate::codegen_error::CodegenError;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CompilerBackend
{
    C,
}

impl FromStr for CompilerBackend
{
    type Err = CodegenError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s {
            "c" | "C" => Ok(Self::C),
            _ => Err(CodegenError::InvalidBackend),
        }
    }
}

pub const fn get_compiler_fn(backend: &CompilerBackend) -> fn(&IRBlock) -> String
{
    match *backend {
        CompilerBackend::C => c_backend::compile_to_c,
    }
}
