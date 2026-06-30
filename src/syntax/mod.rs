mod ast;
mod convert;
mod diagnostics;
mod lexer;
mod parser;

pub use ast::*;
pub use convert::*;

use crate::types::{AgentFile, SkillFile};
use anyhow::Result;
use std::path::Path;

pub fn parse_native_agent(path: &Path) -> Result<AgentFile> {
    let content = std::fs::read_to_string(path)?;
    let ast = crate::syntax::parser::parse_agent(&content)?;
    convert_ast_to_agent(&ast)
}

pub fn parse_native_skill(path: &Path) -> Result<SkillFile> {
    let content = std::fs::read_to_string(path)?;
    let ast = crate::syntax::parser::parse_skill(&content)?;
    convert_ast_to_skill(&ast)
}

pub fn is_native_syntax(content: &str) -> bool {
    content.starts_with("agent \"")
        || content.starts_with("skill \"")
        || content.contains("\npermissions {")
        || content.contains("\nskill \"")
        || content.contains("agent \"")
}
