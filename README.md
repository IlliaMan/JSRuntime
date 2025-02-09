# JS Runtime

This project implements a JS runtime that supports a subset of JS.

## Features

- **Arithmetic Operations**
  - Basic operators: `+`, `-`, `*`, `/`
  - Operator precedence: `*` and `/` before `+` and `-`
  - Parentheses for grouping: `(2 + 3) * 4`

- **Unary Operations**
  - Negative numbers: `-5`, `-(-10)`

- **Variable Declarations**
  - `let` with optional initializer: `let x = 5;`
  - `const` with required initializer: `const PI = 3.14;`

- **Basic Type System**
  - Number type (double-precision floating point)
  - Variable identifiers
  
- **Execution Model**
  - Mandatory semicolons after statements
  - Single file execution model
  - Sequential top-to-bottom execution

- **Constraints**
  - Variable reassignment is not supported
  - Semicolon must be added at the end of statements
  - Comments are not supported

## Usage

```bash
cargo run -- <path_to_js_file>
```

## Supported JS Subset Example

```js
const ANSWER = 42;
let result = (10 + 5) * 2;

let calculation = 50 - -ANSWER / 2;
let group_test = (3 + 5) * (7 - 2);
```

## Language Grammar

The currently implemented grammar (will be updated as features are added):

```bash
PROGRAM -> STATEMENT* TokenType::Eof

STATEMENT -> DECLARATION | EXPRESSION_STATEMENT

DECLARATION -> (TokenType::KeywordLet | TokenType::KeywordConst)
               IDENTIFIER (TokenType::Equals EXPRESSION)? 
               TokenType::Semicolon

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

# Roadmap

- Primitive types: string, boolean, null
- Reference types: array, object
- Operators: comparison, string, logical, ternary, type, bitwise, unary
- Function declaration
- Control flow (if/else statements)
- Automatic semicolon insertion (ASI)
- Comments
- Variable reassignment
- Async support
- ES6 module support