# ckek

KEKEKEKEK
TODO

## Syntax

Function parameters will be handed over as a "function-like" value to the function, so can dont have to be evaluated.
The code consists of a list of expressions that are always evaluated in order.

- `<fn><args...>` Call a function
- `(<args>,<expr...>)` Function expression.
- `(<expr...>)` Function expression with no arguments, also used like brackets.

## Functions and Variables

### Names

| Name  | params  | Explanation / Usage                                                        |
| ----- | ------- | -------------------------------------------------------------------------- |
| `0-9` | ``      |                                                                            |
| `.`   | `a`     | Output / standart output function                                          |
| `^`   | `a`     | Return                                                                     |
| `,`   | `a`     | Input / Read one char into a variable                                      |
| `=`   | `?aa`   | Assign. Assigns the second argument to the first (no-op if not possible)   |
| `+`   | `aa`    | Add / List Append                                                          |
| `-`   | `aa`    | Subtract / Remove all element with that value                              |
| `/`   | `aa`    | Divide                                                                     |
| `*`   | `aa`    | Multiply / List Repeat                                                     |
| `%`   | `aa`    | Modulo / List retain that `(x,#x)`                                         |
| `<`   | `aa`    | Bit shift left / Bit shift left / List shift                               |
| `>`   | `aa`    | Bit shift right / Bit shift right / List pop                               |
| ` `   | `aa`    | Binary or                                                                  |
| `/`   | `aa`    | Or / If the first arg is false, return the second                          |
| ` `   | `aa`    | Binary                                                                     |
| `&`   | `aa`    | And / If the first arg. is true, return the second                         |
| `¬`   | `a`     | Binary not                                                                 |
| `!`   | `a`     | Not                                                                        |
| `#`   | `a`     | Inverted Not                                                               |
| `»`   | `a`     | Greater 0 / Returns whether the arg is greater than 0                      |
| `«`   | `a`     | Smaller 0 / Returns whether the arg is smaller than 0                      |
| `'`   | `?a`    | Increment                                                                  |
| `"`   | `?a`    | Decrement                                                                  |
| `:`   | `?vl$c` | For of. Returns array of every evaluated value                             |
| `;`   | `?vl$c` | For of. Returns the last iteration                                         |
| `_`   | `n`     | Range of all integers up to n. Enumerates a list                           |
| `[`   | `li`    | Forward Index -> pointer                                                   |
| `]`   | `li`    | Backward Index -> pointer                                                  |
| `°`   | `l`     | Absolute value / Length of list                                            |
| `@`   | `cc`    | No-op that just evals boths args and returns the second                    |
| `§`   | `?a?a`  | Packs both arguments in a list. Also supports assignment for destructuring |

### Varible "Modifiers"

- `$` Threads the following function like a variable. Usefull to assign or pass them as a argument. Alternatively it also enables multi-evaluation of function arguments if used as a prefix in the declaration. Also used the end number literals
- `?` In a literal function expression, this makes parameters in-out or inverts this behavior in function evokation; In-out variable will not work if the output in the function invokation is not

### Behavior

Functions return the value of the last express before it ends or reaches a return symbol
Arguments of functions are only evaluated if needed.

### Types

- Function
- List<T>
- Number
- undefined
- (String primitives that behave like List<Number>)

## Example
