# Functionality

## Literals

Literals can be expressed in a few different ways:

- Integers: `10`, `6`, etc
- Decimals: `2.2`, `10.9`, etc
- Scientific notation: `1e5`, `10e6`, etc

## Variables

Variables can be used to simplify your expressions. Variable definitions can be created like: `my_var: <expression>`. When you're creating variables the variable definition must be the first thing on that line. To later use the variable simply use the variable name directly. Any given line can only reference variables defined on lines above it.

Example:
```
my_var: 5     |     5
2 * my_var    |    10
```

## Operations

The infix operators respect order of operations, and parentheses can be used to get a different order of operations.

Currently supported operations:

- Addition: `left + right` or `add(left, right)`
- Subtraction: `left - right`, `sub(left, right)` or `subtract(left, right)`
- Multiplication: `left * right`, `mul(left, right)` or `multiply(left, right)`
- Division: `left / right`, `div(left, right)` or `divide(left, right)`
- Remainder: `left % right`, `mod(left, right)` or `modulus(left, right)`
- Power: `left ^ right`, `pow(left, right)` or `power(left, right)`
- Min: `min(left, right)` or `minimum(left, right)`
- Max: `max(left, right)` or `maximum(left, right)`
- Square root: `sqrt(arg)`, `squareroot(arg)` or `square_root(arg)`
- Factorial: `fact(arg)` or `factorial(arg)`