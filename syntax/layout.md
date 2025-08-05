## Categories

must be one of:
//     Operator - an operation between two values, e.g. +, *, ^ 
//     Function - a function of 1 or more arguments, e.g. sin, ln
//     Constant - a named mathematical constant such as pi or e
//     Literal - a number such as 2, -0.5 etc
//     Variable - placeholder for a value that can be changed for each evaluation
//     OpenBracket - opening parenthesis
//     CloseBracket -  closing parenthesis
//     Separator - tokens that are required by the syntax but have no direct affect, for example the separator between function arguments

Each category can only have one entry
Variable is only required category and must contain exactly one rule

## Labels

Must be unique
Custom bindings can be passed to Parser::new

```json
[
    {
        // required
        "category": "Operator",
        // required if `category` is Operator or Function and not defined for any rule - defines order operations are resolved
        "precedence": 3,
        // optional - defines order that operations with the same precedence are resolved, defaults to LeftToRight
        "associativity": "LeftToRight",
        // required - a list of categories that may appear directly after this token
        "follows": ["Function", "Literal", "Constant"],
        // required - a list of categories that may appear directly before this token
        "precedes": ["Operator"],
        // required, list of specific rules belonging to this category
        "rules": [
            {
                // regex pattern to match. any capture groups are treated as subsequent tokens and will be parsed next
                "pattern": "+",
                // required if category is Function, Operator or Constant - unique label for code function associated with this operation or constant
                "label": "Add",
                // optional if category is Operator or Function and not defined for category - overrides the same field of category if present
                "precedence": 3,
                // optional - overrides the same field of category if present
                "associativity": "LeftToRight",
                // optional - overrides the same field of category if present
                "follows": ["Literal", "Constant"],
                // optional - overrides the same field of category if present
                "precedes": ["Operator", "Function"]
            }
        ]
    },
    ...
]
```