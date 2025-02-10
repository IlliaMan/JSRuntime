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
  - Primitive types
    - Number type (double-precision floating point)
    - String type
    - Boolean type
    - Null
    - Undefined
  - Variable identifiers
  
- **Execution Model**
  - Mandatory semicolons after statements
  - Single file execution model
  - Sequential top-to-bottom execution

- **Constraints**
  - Variable reassignment is not supported
  - Semicolon must be added at the end of statements

## Usage

```bash
cargo run -- <path_to_js_file>
```

## Supported JS Subset Example

```js
// Variable declarations
const ANSWER = 42;
let result = (10 + 5) * 2;

// Arithmetic operations
let calculation /* block comment */ = 50 - -ANSWER / 2;
let group_test = (3 + 5) * (7 - 2);

let x = false;
let y = true;
const z = 'hello';
let b = null;
const a = "HELLO";
let c = undefined;
```

## Language Grammar

The currently implemented grammar (will be updated as features are added):

```bash
PROGRAM -> STATEMENT* TokenType::Eof

STATEMENT -> DECLARATION | EXPRESSION_STATEMENT

DECLARATION -> (TokenType::KeywordLet | TokenType::KeywordConst)
               IDENTIFIER (TokenType::Assign COMPARISON)? 
               TokenType::Semicolon

EXPRESSION_STATEMENT -> COMPARISON TokenType::Semicolon

COMPARISON -> EXPRESSION (COMPARISON_OPERATOR EXPRESSION)*

EXPRESSION -> TERM ((TokenType::Plus | TokenType::Minus) TERM)*

TERM -> FACTOR ((TokenType::Star | TokenType::Division) FACTOR)*

FACTOR -> LITERAL | IDENTIFIER | UNARY | GROUPING

LITERAL -> TokenType::Number | TokenType::String | TokeType::Boolean | TokenType::Null | TokenType::Undefined

GROUPING -> TokenType::LeftParen EXPRESSION TokenType::RightParen

UNARY -> TokenType::Minus FACTOR 

OPERATOR -> TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash

COMPARISON_OPERATOR -> TokenType::Equal | TokenType::NotEqual |
              TokenType::StrictEqual | TokenType::StrictNotEqual |
              TokenType::GreaterThan | TokenType::GreaterThanOrEqual |
              TokenType::LessThanorEqual | | TokenType::LessThan

IDENTIFIER -> TokenType::Identifier
```

# Roadmap

- Reference types: array, object
- Operators: comparison, string, logical, ternary, type, bitwise, unary
- Function declaration
- Control flow (if/else statements)
- Automatic semicolon insertion (ASI)
- Variable reassignment
- Async support
- ES6 module support