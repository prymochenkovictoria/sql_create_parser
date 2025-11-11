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
* `NOT NULL` constraint

The parsing process uses a `pest` grammar to generate a parse tree, which is then traversed to build a strongly-typed AST (`CreateTableQuery`). This AST can be used for schema validation, code generation, or database migration analysis.

## Features
* Parse SQL CREATE TABLE Queries (including `NOT NULL`).
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
    * `not_null`: A `bool` indicating if the `NOT NULL` constraint is present.
* **DataType** (enum):
    * `Simple(String)`: e.g., INT, TEXT.
    * `Varchar(u64)`: e.g., VARCHAR(255).

## Links
[Crate on crates.io](https://crates.io/crates/sql_create_parser)  
[Documentation on docs.rs](https://docs.rs/sql_create_parser/latest/sql_create_parser/)    

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
    username VARCHAR(100) NOT NULL,
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
            not_null: false,
        },
        ColumnDef {
            name: "username",
            data_type: Varchar(
                100,
            ),
            not_null: true,
        },
        ColumnDef {
            name: "email",
            data_type: Simple(
                "TEXT",
            ),
            not_null: false,
        },
        ColumnDef {
            name: "is_active",
            data_type: Simple(
                "BOOLEAN",
            ),
            not_null: false,
        },
        ColumnDef {
            name: "created_at",
            data_type: Simple(
                "DATE",
            ),
            not_null: false,
        },
    ],
}
```

## Grammar
```
WHITESPACE = _{" " | "\t" | "\r" | "\n"}

CREATE = {"CREATE" | "create"}
TABLE = {"TABLE" | "table"}

NOT = {"NOT" | "not"}
NULL = {"NULL" | "null"}

identifier = @{(ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")*}

number = @{ASCII_DIGIT+}

simple_type = @{"INT" | "int" | "TEXT" | "text" | "BOOLEAN" | "boolean" | "DATE" | "date"}

varchar_type = {("VARCHAR" | "varchar") ~ "(" ~ number ~ ")"}

data_type = {simple_type | varchar_type}

not_null_constraint = {(NOT ~ NULL)}

column_definition = {identifier ~ data_type ~ not_null_constraint?}

column_list = {column_definition ~ ("," ~ column_definition)*}

create_query = {SOI ~ CREATE ~ TABLE ~ identifier ~ "(" ~ column_list ~ ")" ~ (";")? ~ EOI}
```