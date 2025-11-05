#![doc = include_str!("../docs.md")]

use ::pest_derive::Parser;
use pest::iterators::Pair;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SQLParser;

/// Represents a SQL Data Type.
#[derive(Debug, PartialEq)]
pub enum DataType {
    Simple(String),
    Varchar(u64),
}

/// Represents a single column definition.
#[derive(Debug, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
}

/// Main structure for a CREATE TABLE query.
#[derive(Debug, PartialEq)]
pub struct CreateTableQuery {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
}

// Error Handling
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("No query found")]
    NoQueryFound,
    #[error("Missing table name")]
    MissingTableName,
    #[error("Invalid column definition")]
    InvalidColumnDefinition,
    #[error("Invalid data type")]
    InvalidDataType,
    #[error("Invalid VARCHAR length: {0}")]
    InvalidVarcharLength(#[from] std::num::ParseIntError),
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),
}

/// Function to parse a SQL CREATE TABLE query.
pub fn parse_query(input: &str) -> Result<CreateTableQuery, ParseError> {
    let mut pairs = SQLParser::parse(Rule::create_query, input)
        .map_err(|e| ParseError::ParsingError(e.to_string()))?;
    let pair = pairs.next().ok_or(ParseError::NoQueryFound)?;
    build_query_structure(pair)
}

/// Builds the CreateTableQuery AST from the parsed pairs.
fn build_query_structure(pair: Pair<Rule>) -> Result<CreateTableQuery, ParseError> {
    let mut table_name: Option<String> = None;
    let mut columns: Vec<ColumnDef> = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                table_name = Some(inner.as_str().to_string());
            }
            Rule::column_list => {
                columns = parse_column_list(inner)?;
            }
            Rule::CREATE | Rule::TABLE | Rule::EOI => {}
            _ => return Err(ParseError::UnexpectedRule(inner.as_rule())),
        }
    }

    Ok(CreateTableQuery {
        table_name: table_name.ok_or(ParseError::MissingTableName)?,
        columns,
    })
}

/// Parses the list of column definitions.
fn parse_column_list(pair: Pair<Rule>) -> Result<Vec<ColumnDef>, ParseError> {
    pair.into_inner()
        .map(parse_column_definition)
        .collect()
}

/// Parses a single column definition.
fn parse_column_definition(pair: Pair<Rule>) -> Result<ColumnDef, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or(ParseError::InvalidColumnDefinition)?.as_str().to_string();
    let data_type_pair = inner.next().ok_or(ParseError::InvalidColumnDefinition)?;
    let data_type = parse_data_type(data_type_pair)?;
    Ok(ColumnDef { name, data_type })
}

/// Parses a data type rule into the DataType enum.
fn parse_data_type(pair: Pair<Rule>) -> Result<DataType, ParseError> {
    let inner = pair.into_inner().next().ok_or(ParseError::InvalidDataType)?;
    match inner.as_rule() {
        Rule::simple_type => Ok(DataType::Simple(inner.as_str().to_uppercase())),
        Rule::varchar_type => {
            let num_str = inner.into_inner().next().ok_or(ParseError::InvalidDataType)?.as_str();
            let len = num_str.parse::<u64>()?;
            Ok(DataType::Varchar(len))
        }
        _ => Err(ParseError::UnexpectedRule(inner.as_rule())),
    }
}