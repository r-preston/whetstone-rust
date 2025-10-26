# Whetstone
Flexible crate for parsing and evaluating strings representing equations.

### Easy interface
- Parse an equation in two lines of code
### Extreme flexibility
- Supports user-defined syntax, functions and numeric types
- Customise built-in syntax or create your own from scratch with easy JSON syntax
### Plug and play with built-in syntax
- Standard
- LaTeX

## Roadmap
- Support for wrapping functions/operators like |...|
- Support for capturing functions like \sqrt[n]{...}
- Add support for integration, differentiation and summation

## Usage

```rust
use whetstone::{Parser, syntax::Syntax};

// Requires type: any type that satisfies trait whetstone::NumericType.
// May return an error if there is an issue with the provided syntax
let factory = Parser::<f32>::new(Syntax::Standard).unwrap();
// May return an error if the string is not a valid equation
let equation = factory.parse("sin(x) + 2^x").unwrap();
// May return error if variable not found or cannot be mutably borrowed
*equation.variable("x").unwrap() = 1.0; // set value of x to 1
// May return error if semantic or math error occurs
let value = equation.evaluate().unwrap(); // 2.841470984807897
```

The variables detected during parsing can be seen by

## Syntax

Two built in syntax rulesets: standard and LaTeX.
Definitions for both can be found in `src/syntax/json/`.

## Custom syntax

Rules are defined for one of the following categories.

Categories | Description
--|--
Functions | A function name followed by a number of arguments
Operators | An operation between two arguments
ImplicitOperators | An operation implied by the context of two arguments
Constants | A pattern representing a constant value
Literals | A literal number
Variables | Pattern for matching variables
OpenBrackets | An opening or left parenthesis
CloseBrackets | An closing or right parenthesis
Separators | Separator between function arguments
Fluff | Tokens without any syntactic meaning, may appear anywhere and are ignored

Custom syntax may be defined in one of two ways:

### JSON
Create a parser from a JSON string of rule definitions using `Parser<T>::from_json(&str)`.

```json
{
    // one block for each category
    "Operators": {
        // required - a list of categories that may appear directly before this token
        "may_follow": ["Function", "Literal", "Constant"],
        // required if `category` is Operator or Function and not defined for any rule - defines order operations are resolved
        "default_precedence": 3,
        // optional - defines order that operations with the same precedence are resolved, defaults to LeftToRight
        "default_associativity": "LeftToRight",
        // required, list of specific rules belonging to this category
        "rules": [
            {
                // Pattern to match for this rule
                "pattern": "+",
                // If true, pattern is treated as a regex expression. Default: false
                "pattern_is_regex": true,
                // required if category is Function, Operator or Constant - unique label for code function associated with this operation or constant
                "binding": "Add",
                // required if category is Operator or Function and not defined for category - overrides the same field of category if present
                "precedence": 3,
                // optional - overrides the same field of category if present
                "associativity": "LeftToRight",
                // optional - overrides the same field of category if present
                "may_follow": ["Literal", "Constant"],
                // integer currently used for brackets to identify which left and right brackets are pairs. matching brackets should have the same context value
                "context": 1
            },
            ...
        ]
    },
    ...
}
```

### RuleCollectionDefinition

Used with `Parser<T>::from_definitions(RuleCollectionDefinition)`.

Can be constructed manually or modifed from a built-in syntax using `syntax::copy_definition(syntax: &Syntax)` to first copy the structure.


## Custom bindings

`whetstone` comes with a selection of built-in bindings for common functions and constants (see below), but also supports user-defined bindings.

All existing bindings can be obtained by calling `bindings::get_bindings::<f32>()`.

Custom functions must be defined in the following form:
```rust
use whetstone::{NumericType, Value};

fn custom_function<T: NumericType>(args: &[T]) -> Value<T> {
    Ok(args[0] * (1.0 / args[1]))
}
```

Functions can be added as bindings by first constructing a static slice containing the function, number of arguments and the label, like so:
```rust
use whetstone::bindings::FunctionPointer;

static MY_FUNCTIONS: &[(&str, FunctionPointer<f32>, usize)] = &[("NewBinding", custom_function, 2)];
```
This slice can then be passed to `bindings::register_bindings(MY_FUNCTIONS)`, which adds the bindings. The label `"NewBinding"` can now be used in syntax rule definitions.

## Built in Constants

The following mathematical constants are built-in and can be referenced in rule definitions using their label.

Label | Description
-- | --
Pi | $\pi$
Euler | $e$, Euler's number

## Built-in Functions

The following mathematical functions are built-in and can be referenced in rule definitions using their label.

Label | Number of arguments | Description
-- | -- | --
Add | 2 | $a+b$
Subtract | 2 | $a - b$
Multiply | 2 | $a\times b$
Divide | 2 | $\frac{a}{b}$
Exponent | 2 | $a^b$
Modulo | 2 | Returns remainder of division
Min | 2 | Returns smaller of two numbers
Max | 2 | Returns greater of two numbers
Negate | 1 | $-1 * x$
SquareRoot | 1 | $\sqrt{x}$
LogBaseE | 1 | Natural logarithm
LogBase10 | 1 | Logarithm to base 10
Absolute | 1 | Absolute magnitude of a number
Round | 1 | Round to nearest integer
Ceiling | 1 | Round up to next integer
Floor | 1 | Round down to next integer
Sine | 1 |
Cosine | 1 |
Tangent | 1 |
Arcsine | 1 |
Arccosine | 1 |
Arctangent | 1 |
Cosecant | 1 |
Secant | 1 |
Cotangent | 1 |
HypSine | 1 |
HypCosine | 1 |
HypTangent | 1 |
InvHypSine | 1 |
InvHypCosine | 1 |
InvHypTangent | 1 |