use thiserror::Error;

use super::*;

#[derive(Debug, Error)]
pub enum ParseError {
}

pub fn parse_stmt(input: &str) -> Result<Stmt, ParseError> {
    todo!()
}
