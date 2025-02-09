# JS Runtime

This project implements a JS runtime that supports a subset of JS.

- **Subset of JavaScript:** Supports a limited set of JS features.
- **Semicolon:** It must be at the end of every statement.
- **Single File Execution:** Only supports one JavaScript file at a time—no support for modules.

## Usage
```bash
cargo run -- <path_to_js_file>
```

## Language Grammar

Here, it is the currently implemented grammar. It will be updated accordingly to gradually added features.

Nonterminals are UPPER Case, while Terminals are lower case and enum TokenType members. PROGRAM is the start symbol.


```bash
# Productions (Simplified Grammar)

PROGRAM -> STATEMENT* TokenType::Eof

STATEMENT -> DECLARATION | EXPRESSION_STATEMENT

DECLARATION -> (TokenType::KeywordLet | TokenType::KeywordConst) IDENTIFIER (TokenType::Equals EXPRESSION)? TokenType::Semicolon

EXPRESSION_STATEMENT -> EXPRESSION TokenType::Semicolon

EXPRESSION -> TERM ((TokenType::Plus | TokenType::Minus) TERM)*

TERM -> FACTOR ((TokenType::Star | TokenType::Division) FACTOR)*

FACTOR -> LITERAL | IDENTIFIER | UNARY | GROUPING

LITERAL -> TokenType::Number

GROUPING -> TokenType::LeftParen EXPRESSION TokenType::RightParen

UNARY -> TokenType::Minus FACTOR 

OPERATOR -> TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash

IDENTIFIER -> TokenType::Identifier
```
