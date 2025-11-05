use anyhow::{Context, Result};
use sql_create_parser::{parse_query, ColumnDef, CreateTableQuery, DataType};

#[test]
fn test_simple_create_table() -> Result<()> {
    let query = "CREATE TABLE users (id INT, name TEXT);";
    let parsed = parse_query(query).context("Failed to parse simple create table")?;

    let expected = CreateTableQuery {
        table_name: "users".to_string(),
        columns: vec![
            ColumnDef {
                name: "id".to_string(),
                data_type: DataType::Simple("INT".to_string()),
            },
            ColumnDef {
                name: "name".to_string(),
                data_type: DataType::Simple("TEXT".to_string()),
            },
        ],
    };

    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn test_varchar_type() -> Result<()> {
    let query = "CREATE TABLE products (sku VARCHAR(255), price INT)";
    let parsed = parse_query(query).context("Failed to parse varchar type")?;

    assert_eq!(parsed.table_name, "products");
    assert_eq!(parsed.columns.len(), 2);
    assert_eq!(
        parsed.columns[0],
        ColumnDef {
            name: "sku".to_string(),
            data_type: DataType::Varchar(255),
        }
    );
    Ok(())
}

#[test]
fn test_invalid_query_missing_parens() {
    let query = "CREATE TABLE users;";
    let result = parse_query(query);
    assert!(
        result.is_err(),
        "Expected parsing to fail due to missing parentheses"
    );
}

#[test]
fn test_invalid_data_type() {
    let query = "CREATE TABLE users (id FOOBAR)";
    let result = parse_query(query);
    assert!(
        result.is_err(),
        "Expected parsing to fail due to invalid data type"
    );
}