# sql_create_parser

## Overview
A command-line tool designed to parse and analyze SQL CREATE TABLE queries. Developed in Rust, it uses the pest parser generator to produce a well-structured Abstract Syntax Tree (AST) that represents the input query.

## Technical Description
This tool parses `CREATE TABLE` statements. The parser identifies the table name and a list of column definitions. Each column definition consists of a column name and its corresponding data type.

Supported data types:
* `INT`
* `TEXT`
* `BOOLEAN`
* `DATE`
* `VARCHAR(n)` (where 'n' is a number)

The parsing process uses a `pest` grammar to generate a parse tree, which is then traversed to build a strongly-typed AST (`CreateTableQuery`). This AST can be used for schema validation, code generation, or database migration analysis.

## Features
* Parse SQL CREATE TABLE Queries.
* Abstract Syntax Tree (AST) Generation.
* Simple Command-Line Interface.
* Detailed error handling with `thiserror`.
* Includes `Makefile` for easy building, testing, and linting.

## AST (Data Structures)
* **CreateTableQuery**:
    * `table_name`: The name of the table.
    * `columns`: A `Vec<ColumnDef>` of column definitions.
* **ColumnDef**:
    * `name`: The name of the column.
    * `data_type`: A `DataType` enum.
* **DataType** (enum):
    * `Simple(String)`: e.g., INT, TEXT.
    * `Varchar(u64)`: e.g., VARCHAR(255).

## LINKS
[Crate on crates.io](https://crates.io/crates/sql_create_parser)  
[Documentation on docs.rs](https://docs.rs/sql_create_parser/0.1.0/sql_create_parser/)    

## Installation
Clone the repository:
```unix
git clone https://github.com/prymochenkovictoria/sql_create_parser
cd sql_create_parser
```

## Example 
We have an example file: 
```
CREATE TABLE users (
    user_id INT,
    username VARCHAR(100),
    email TEXT,
    is_active BOOLEAN,
    created_at DATE
);
```
It will be parsed like this:
```
Parsed Query:
CreateTableQuery {
    table_name: "users",
    columns: [
        ColumnDef {
            name: "user_id",
            data_type: Simple(
                "INT",
            ),
        },
        ColumnDef {
            name: "username",
            data_type: Varchar(
                100,
            ),
        },
        ColumnDef {
            name: "email",
            data_type: Simple(
                "TEXT",
            ),
        },
        ColumnDef {
            name: "is_active",
            data_type: Simple(
                "BOOLEAN",
            ),
        },
        ColumnDef {
            name: "created_at",
            data_type: Simple(
                "DATE",
            ),
        },
    ],
}
```