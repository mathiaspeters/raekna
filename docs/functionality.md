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

### Arithmetic

All arithmetic operations can either be used through infix operators or function calls. Infix operators obey the order of operations and parentheses can be used to change the order of operations.

**Addition**

Simple addition.
```
add(left, right)
left + right
```

**Subtraction**

Simple subtraction.
```
sub(left, right)
subtract(left, right)
left - right
```

**Negation**

Negates a number.
```
neg(arg)
negate(arg)
-arg
```

**Multiplication**

Simple multiplication.
```
mul(left, right)
multiply(left, right)
left * right
```

**Division**

Simple division. The divisor has to be a non-zero number.
```
div(dividend, divisor)
divide(dividend, divisor)
dividend / divisor
```

**Remainder**

Calculates the remainder of `dividend/divisor`. The divisor has to be a non-zero number.
```
mod(dividend, divisor)
modulus(dividend, divisor)
dividend % divisor
```

**Power**

Calculates the base the the power of the exponent.
```
pow(base, exponent)
power(base, exponent)
base ^ exponent
```

### Comparisons

**Minimum**

Returns the smaller value of the inputs.
```
min(left, right)
minimum(left, right)
```

**Maximum**

Returns the larger value of the inputs.
```
max(left, right)
maximum(left, right)
```

### Rounding

Ceil/CeilPrec
Floor/FloorPrec
Round/RoundPrec
Trunc/TruncPrec

### Trigonometry

Sin
Cos
Tan
SinH
CosH
TanH
ArcSin
ArcCos
ArcTan
ArcSinH
ArcCosH
ArcTanH

### Miscellaneous math

**Square root**

Computes the square root of a number. The argument has to be a positive number.
```
sqrt(arg)
squareroot(arg)
square_root(arg)
```

**Cube root**

Computes the cube root of a number.
```
cbrt(arg)
cuberoot(arg)
cube_root(arg)
```

**Factorial**

Computes the factorial of a number. The argument has to be a positive integer and cannot be larger than 20, as it would otherwise exceed the size of a signed 64-bit integer.
```
fact(arg)
factorial(arg)
```

**Log**

Computes the logarithm of a value with a given base. The given value must be larger than 0 and the base must be 0 or larger.
```
log(value, base)
```

**Log2**

Computes the logarithm of a value with a base of 2. The given value must be larger than 0.
```
log2(arg)
```

**Log10**

Computes the logarithm of a value with a base of 10. The given value must be larger than 0.
```
log10(arg)
```

**Ln**

Computes the natural logarithm of a value. The given value must be larger than 0.
```
ln(arg)
```

**Abs**

Returns the absolute value of a number.
```
abs(arg)
```