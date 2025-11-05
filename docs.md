# sql_create_parser

## Purpose
`sql_create_parser` is used for parsing SQL CREATE TABLE queries into a structured Abstract Syntax Tree (AST) using Rust and pest.

This parser analyzes `CREATE TABLE` syntax. It identifies the table name and a list of column definitions. Each definition consists of a column name and its data type (e.g., `INT`, `TEXT`, `VARCHAR(255)`). The resulting AST (`CreateTableQuery`) can be used for schema validation or code generation.

## Grammar
1.  **WHITESPACE**: Matches spaces, tabs, and newlines (ignored).
2.  **Keywords**: `CREATE`, `TABLE`.
3.  **identifier**: Matches table and column names.
4.  **number**: Matches digits (for VARCHAR length).
5.  **simple_type**: Matches types like `INT`, `TEXT`, `BOOLEAN`, `DATE`.
6.  **varchar_type**: Matches `VARCHAR(n)`.
7.  **data_type**: A rule that combines `simple_type` and `varchar_type`.
8.  **column_definition**: Matches a column name and its `data_type`.
9.  **column_list**: Matches one or more `column_definition`s separated by commas.
10. **create_query**: The main rule, defines the full `CREATE TABLE` statement structure.