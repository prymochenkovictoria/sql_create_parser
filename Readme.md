# sql_create_parser

## Overview
A command-line tool designed to parse and analyze SQL CREATE TABLE queries. Developed in Rust, it uses the pest parser generator to produce a well-structured Abstract Syntax Tree (AST) that represents the input query.

## Technical Description
This tool parses `CREATE TABLE` statements. The parser identifies the table name and a list of column definitions using a `pest` grammar.

Each column definition is broken down into its **name** and **data type** (e.g., `INT`, `TEXT`, `VARCHAR(255)`).

The parsing process generates a Rust struct (`CreateTableQuery`) that mirrors the query's structure. This AST can then be used for:
* Validating database schema syntax.
* Tools for analyzing or migrating database structures.
* Automatic code generation.

## Grammar
The core grammar rule (`create_query`) defines the expected structure:
```pest
create_query = { SOI ~ CREATE ~ TABLE ~ identifier ~ "(" ~ column_list ~ ")" ~ (";")? ~ EOI }