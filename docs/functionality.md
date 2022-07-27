# Functionality

## Limitations

The current architecture limits numeric values to what a 64-bit floating point value can hold, which has a lower bound of `-1.7976931348623157e308` and an upper bound of `1.7976931348623157e308`. There are plans to allow for much bigger numbers eventually but for now you will get an error if your literals or the results of your calculations exceed those bounds. Similarly you will get an error if you are trying to perform an invalid calculation, like getting the logarithm or square root of a negative number. 

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

## Constants

Raekna supports some built-in mathematical constants, currently `pi`, `tau` and `e`.
```
pi      | 3.1415...
tau     | 6.2831...
e       | 2.7182...
```

The constants are case-insensitive so `pi`, `PI`, `Pi` will all return the value of `pi`.

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

All rounding functions will assume you are trying to round the the relevant integer value, but you can customize the result. All rounding functions accept a precision as an integer value that will determine how many decimals to include. Additionally, `ceil`, `floor`, and `round` can instead accept a stepping value as a floating point value and will round to relevant multiple of that stepping.

**Ceil**

```
ceil(value)
ceil(value, precision)
ceil(value, stepping)

ceil(5.4)       | 6         # Normal ceil
ceil(1.2345, 2) | 1.24      # Ceil with precision
ceil(1, 2.4)    | 2.4       # Ceil with stepping
```

**Floor**

```
floor(value)
floor(value, precision)
floor(value, stepping)

floor(5.9)       | 5         # Normal floor
floor(9.8765, 2) | 9.87      # Floor with precision
floor(4.7, 2.4)  | 2.4       # Floor with stepping
```

**Round**

```
round(value)
round(value, precision)
round(value, stepping)

round(5.9)       | 6         # Normal round
round(5.4)       | 5         # Normal round
round(9.8765, 2) | 9.88      # Round with precision
round(9.8735, 2) | 9.87      # Round with precision
round(4.9, 2.4)  | 4.8       # Round with stepping
round(4.7, 2.4)  | 2.4       # Round with stepping
```

**Trunc**

```
trunc(value)
trunc(value, precision)

trunc(5.9)       | 5         # Normal trunc
trunc(9.8765, 2) | 9.87      # Trunc with precision
trunc(4.7, 2.4)  | Error     # Trunc with stepping
```


### Trigonometry

All trigonometric functions work with radians.

**Sin**

Computes the sine of the given argument.
```
sin(arg)
```

**Cos**

Computes the cosine of the given argument.
```
cos(arg)
```

**Tan**

Computes the tangent of the given number.
```
tan(arg)
```

**SinH**

Hyperbolic sine function.
```
sinh(arg)
```

**CosH**

Hyperbolic cosine function.
```
cosh(arg)
```

**TanH**

Hyperbolic tangent function.
```
tanh(arg)
```

**ArcSin**

Computes the arcsine of the given argument.
```
asin(arg)
```

**ArcCos**

Computes the arccosine of the given argument.
```
acos(arg)
```

**ArcTan**

Computes the arctangent of the given number.
```
atan(arg)
```

**ArcSinH**

Inverse hyperbolic sine function.
```
asinh(arg)
```

**ArcCosH**

Inverse hyperbolic cosine function.
```
acosh(arg)
```

**ArcTanH**

Inverse hyperbolic tangent function.
```
atanh(arg)
```


### Miscellaneous math

**Square root**

Computes the square root of a number.
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

Computes the logarithm of a value with a given base.
```
log(value, base)
```

**Log2**

Computes the logarithm of a value with a base of 2.
```
log2(arg)
```

**Log10**

Computes the logarithm of a value with a base of 10.
```
log10(arg)
```

**Ln**

Computes the natural logarithm of a value.
```
ln(arg)
```

**Abs**

Returns the absolute value of a number.
```
abs(arg)
```