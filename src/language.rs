//! The database language for creating, inserting, deleting, and selecting database objects and
//! records

mod parser;

pub use parser::ParseError;

use std::str::FromStr;
use std::sync::Arc;

/// A single statement in the database language
#[derive(Debug, Clone)]
pub enum Stmt {
    Create(Create),
    Insert(Insert),
    Select(Select),
}

impl FromStr for Stmt {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::parse_stmt(input)
    }
}

#[derive(Debug, Clone)]
pub enum Create {
    Database(CreateDatabase),
    Table(CreateTable),
}

#[derive(Debug, Clone)]
pub struct CreateDatabase {
    /// The name of the database to create
    pub name: Arc<str>,
}

#[derive(Debug, Clone)]
pub struct CreateTable {
    /// The name of the table to create
    pub name: Arc<str>,
}

#[derive(Debug, Clone)]
pub enum Insert {
}

#[derive(Debug, Clone)]
pub enum Select {
}
