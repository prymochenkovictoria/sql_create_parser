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
WHITESPACE = _{" " | "\t" | "\r" | "\n"}

CREATE = {"CREATE" | "create"}
TABLE = {"TABLE" | "table"}

identifier = @{(ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")*}

number = @{ASCII_DIGIT+}

simple_type = @{"INT" | "int" | "TEXT" | "text" | "BOOLEAN" | "boolean" | "DATE" | "date"}

varchar_type = {("VARCHAR" | "varchar") ~ "(" ~ number ~ ")"}

data_type = {simple_type | varchar_type}

column_definition = {identifier ~ data_type}

column_list = {column_definition ~ ("," ~ column_definition)*}

create_query = {SOI ~ CREATE ~ TABLE ~ identifier ~ "(" ~ column_list ~ ")" ~ (";")? ~ EOI}