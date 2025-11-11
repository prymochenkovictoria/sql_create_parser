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
                not_null: false,
            },
            ColumnDef {
                name: "name".to_string(),
                data_type: DataType::Simple("TEXT".to_string()),
                not_null: false,
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
            not_null: false, 
        }
    );
    assert_eq!(
        parsed.columns[1],
        ColumnDef {
            name: "price".to_string(),
            data_type: DataType::Simple("INT".to_string()),
            not_null: false, 
        }
    );
    Ok(())
}

#[test]
fn test_all_types_lowercase() -> Result<()> {
    let query = "create table logs (id int, msg text, active boolean, tag varchar(50), created_at date);";
    let parsed = parse_query(query).context("Failed to parse all types query")?;

    assert_eq!(parsed.table_name, "logs");
    assert_eq!(parsed.columns.len(), 5);
    
    assert_eq!(parsed.columns[0].not_null, false); 
    assert_eq!(parsed.columns[1].not_null, false); 

    assert_eq!(
        parsed.columns[2].data_type,
        DataType::Simple("BOOLEAN".to_string())
    );
    assert_eq!(
        parsed.columns[3].data_type,
        DataType::Varchar(50)
    );
     assert_eq!(
        parsed.columns[4].data_type,
        DataType::Simple("DATE".to_string())
    );
    Ok(())
}

#[test]
fn test_single_column() -> Result<()> {
    let query = "CREATE TABLE settings (setting_key VARCHAR(100))";
    let parsed = parse_query(query).context("Failed to parse single column")?;

    assert_eq!(parsed.table_name, "settings");
    assert_eq!(parsed.columns.len(), 1);
    assert_eq!(
        parsed.columns[0],
        ColumnDef {
            name: "setting_key".to_string(),
            data_type: DataType::Varchar(100),
            not_null: false, 
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
fn test_invalid_query_missing_comma() {
    let query = "CREATE TABLE users (id INT name TEXT)";
    let result = parse_query(query);
    assert!(
        result.is_err(),
        "Expected parsing to fail due to missing comma"
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

#[test]
fn test_invalid_varchar_length() {
    let query = "CREATE TABLE users (name VARCHAR(abc))";
    let result = parse_query(query);
    assert!(
        result.is_err(),
        "Expected parsing to fail due to non-numeric varchar length"
    );
}

#[test]
fn test_not_null_constraint() -> Result<()> {
    let query = "CREATE TABLE users (id INT NOT NULL, name TEXT, email VARCHAR(100) not null);";
    let parsed = parse_query(query).context("Failed to parse NOT NULL constraint")?;

    assert_eq!(parsed.table_name, "users");
    assert_eq!(parsed.columns.len(), 3);

    assert_eq!(
        parsed.columns[0],
        ColumnDef {
            name: "id".to_string(),
            data_type: DataType::Simple("INT".to_string()),
            not_null: true,
        }
    );

    assert_eq!(
        parsed.columns[1],
        ColumnDef {
            name: "name".to_string(),
            data_type: DataType::Simple("TEXT".to_string()),
            not_null: false, 
        }
    );

    assert_eq!(
        parsed.columns[2],
        ColumnDef {
            name: "email".to_string(),
            data_type: DataType::Varchar(100),
            not_null: true, 
        }
    );

    Ok(())
}