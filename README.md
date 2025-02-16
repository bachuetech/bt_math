# Project Title
BT MATH

## Description
A simple implementation of an expression evaluator that can handle basic arithmetic operations, parentheses, and some mathematical functions that provide a way to evaluate mathematical expressions using RPN (Reverse Polish Notation) implemented in two parts: parsing and evaluation.

Support the use of PI and E (Euler's number), negative numbers or expressions, and the following functions: ln, log2, exp (e^#), asin, acos, atan, sin, cos. tan. abs, sqrt. log10


## Usage
```
let expression = "2 + 3 * 4";
 .......
let f = evaluate_expression(expression).unwrap();
```

## Version History
* 0.1.0
    * Initial Release
* 0.2.0
    * Added PI and E (Euler's number) support. Support for negative numbers. Fix error with log (log was removed) and added log10.
* 0.3.0
    * POW(x,y) is a function now supported. Also fix some negative number issues. Make it case insensitive. 

## License
GPL-3.0-only