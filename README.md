# JS Runtime

This project implements a JS runtime that supports a subset of JS.

## Features

- **Arithmetic Operations**
  - Basic operators: `+`, `-`, `*`, `/`
  - Operator precedence: `*` and `/` before `+` and `-`
  - Parentheses for grouping: `(2 + 3) * 4`

- **Unary Operations**
  - Negative numbers: `-5`, `-(-10)`

- **Comparison Operations**
  - Same-type comparisons for `number`, `string`, `boolean`, `null` and `undefined`
  - `null` and `undefined` comparisons

- **Variable Declarations**
  - `let` with optional initializer: `let x = 5;`
  - `const` with required initializer: `const PI = 3.14;`

- **Functions**
  - Function declarations: `function name(params) { body }`
  - Parameter handling:
    - Missing parameters get `undefined` value
    - Extra arguments are ignored
  - Return statements:
    - Explicit returns with `return value;`
    - Implicit returns with `undefined` for missing returns
  - No function hoisting (must be declared before use)

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

// Comparison operations
let isEqual = x == y;
let isNullOrUndefined = b == c;
let stringComparison = z == a;
let numericComparison = ANSWER > 10;

// Function declarations
function add(x, y) {
  const text = 'add function';
  return x + y;
}

function emptyBody() {}

function simpleReturn() {
  return;
}

// Function calls
let addResult = add(result, 7);
emptyBody();
```

## Language Grammar

The currently implemented grammar (will be updated as features are added):

```
PROGRAM -> STATEMENT* EOF

STATEMENT -> DECLARATION
           | FUNCTION_DECLARATION
           | EXPRESSION_STATEMENT
           | RETURN_STATEMENT

DECLARATION -> ('let' | 'const') IDENTIFIER ('=' COMPARISON)? ';'
FUNCTION_DECLARATION ->  'function' IDENTIFIER '(' FUNCTION_PARAMS? ')' FUNCTION_BODY
RETURN_STATEMENT -> 'return' COMPARISON? ';'
EXPRESSION_STATEMENT -> COMPARISON ';'

FUNCTION_PARAMS -> IDENTIFIER (',' IDENTIFIER)*
FUNCTION_BODY -> '{' (FUNCTION_BODY_CONTENT)* '}'
FUNCTION_BODY_CONTENT -> DECLARATION | EXPRESSION_STATEMENT | RETURN_STATEMENT

COMPARISON -> EXPRESSION (COMPARISON_OPERATOR EXPRESSION)*
EXPRESSION -> TERM (('+' | '-') TERM)*
TERM -> FACTOR (('*' | '/') FACTOR)*
FACTOR -> LITERAL 
        | IDENTIFIER 
        | UNARY 
        | GROUPING 
        | CALL

UNARY -> '-' FACTOR 
GROUPING -> '(' EXPRESSION ')'
CALL -> IDENTIFIER '(' ARGUMENTS? ')'
ARGUMENTS ->  COMPARISON (',' COMPARISON)*

OPERATOR -> '+' | '-' | '*' | '/'
COMPARISON_OPERATOR -> '==' | '!=' | '===' | '!==' | '>' | '>=' | '<' | | '<='

LITERAL -> NUMBER | STRING | BOOLEAN | NULL | UNDEFINED

IDENTIFIER -> '<sequence of characters that are not reserved words>'
NUMBER -> '<number: integer and decimals>'
STRING -> '<sequence of charachters surrounded by ' or " or `>'
BOOLEAN -> true | false
NULL -> null
UNDEFINED -> undefined
EOF -> '<end-of-file>'
```

# Roadmap

- Reference types: array, object
- Operators: comparison (full support), string, logical, ternary, type, bitwise, unary
- Nested functions, closures
- Arrow functions
- Control flow (if/else statements)
- Automatic semicolon insertion (ASI)
- Variable reassignment
- Async support: asynchronous runtime
- ES6 module support