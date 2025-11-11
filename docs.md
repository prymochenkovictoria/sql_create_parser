# sql_create_parser

## Purpose
`sql_create_parser` is used for parsing SQL CREATE TABLE queries into a structured Abstract Syntax Tree (AST) using Rust and pest.

This parser analyzes `CREATE TABLE` syntax. It identifies the table name and a list of column definitions. Each definition consists of a column name and its data type (e.g., `INT`, `TEXT`, `VARCHAR(255)`). The resulting AST (`CreateTableQuery`) can be used for schema validation or code generation.

## Grammar
1.  **WHITESPACE**: Matches spaces, tabs, and newlines (ignored).
2.  **Keywords**: `CREATE`, `TABLE`.
3.  **New Keywords**: `NOT`, `NULL`.
4.  **identifier**: Matches table and column names.
5.  **number**: Matches digits (for VARCHAR length).
6.  **simple_type**: Matches types like `INT`, `TEXT`, `BOOLEAN`, `DATE`.
7.  **varchar_type**: Matches `VARCHAR(n)`.
8.  **data_type**: A rule that combines `simple_type` and `varchar_type`.
9.  **not_null_constraint**: New rule that matches `NOT NULL`.
10. **column_definition**: Matches a column name, its `data_type`, and an optional `not_null_constraint`.
11. **column_list**: Matches one or more `column_definition`s separated by commas.
12. **create_query**: The main rule, defines the full `CREATE TABLE` statement structure.