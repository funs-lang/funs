# IMPORTANT NOTES
- a file is a module
- the `\n` character is the line terminator
- the `;` character is the function/match terminator
- the difference between "variables" and "functions" is the `(args) ->` part

# Stdlib

- `print: str -> unit` -- print a string to the console

# List
- `hd: [T] -> T` -- get the head of a list
- `tl: [T] -> [T]` -- get the tail of a list

## Option
- `Opt<T>` -- a type that can be `Just` or `Nil`
- `Just: T -> Opt<T>` -- create a `Just` value
- `Nil: Opt<T>` -- create a `Nil` value

# Imports
```python
imp test # use: test.<name>
imp test as t # use: t.<name>
imp (
    test as t
    test2
    test3
)
imp f1 of test # use: f1
imp { f1, f2 } of test # use: f1, f2
imp * of test # import all
```

# Native Types

- NOT USED: `char` -- Unicode character
- `unit` -- Unit
- `int` -- 32 bits
- `float` -- 64 bits
- `str` -- Unicode string
- `bool` -- `True` or `False`
- `[T]` -- List of `T`
- `(T, U)` -- Tuple with two elements of type `T` and `U`

```python
# With type annotation
# x_char: char = '👾' # unicode
x_int: int = 1
x_float: float = 1.0
x_bool: bool = True # False
x_str: str = "hello 👾"
x_list: [int] = [1, 2, 3]
x_tuple: (int, str) = (1, "hello")
x_f1: () -> unit = () -> print "hello" ;
x_f2: () -> int = () -> 1 ;
x_f3: (T) -> T = (x) -> x ; # Generic

# Without type annotation
# x_char = 'a' # unicode
x_int = 1
x_float = 1.0
x_bool = true # false
x_str = "hello"
x_list = [1, 2, 3]
x_tuple = (1, "hello")
x_option = Just(1) # Nil
x_f1 = () -> print "hello" ;
x_f2 = () -> 1 ;
x_f3 = (x) -> x ; # Generic
```

# Cursom Types

## Record

```bash
data MyRecord = {
  a: int
  b: mut str
  c: int
  d: mut str
}
record: MyRecord = {a: 1, b: "a", c: 2, d: "b"}
a = record.a
b = record.b
c = record.c
d = record.d
{a, b, c, d} = record
record.a = 3 # Error: record.a is immutable
record.b = "c" # OK
record.c = 4 # Error: record.c is immutable
record.d = "d" # OK
```

## Variant
```bash
data MyVariant =
| First
| Second
| Third(int)
;
first = First
second = Second
third = Third(1)
match_variant: MyVariant -> str = (v) ->
  match v
  | First -> "first"
  | Second -> "second"
  | Third(n) -> "third"
  ;
;
```

# Functions Overview

```python
f_base: (int, int) -> int = (a, b) ->
  square_a: int = a * a
  square_b: int = b * b
  square_a + square_b # return
;

f_match: (int) -> str = (a) ->
  match a
    0 => "zero"
    1 => "one"
    _ => "other"
  ;
;

f2 = (a, b) ->
  if gt a b then
      a_square = a * a
      a_square # return
  else
      b * b # return
;

f3 = (a, b) ->
    f_inner = (a, b) ->
        if gt a b then
          a_square = a * a
          a_square # return
        else
            b * b # return
    ;
    f_inner a b
;

f4 = (a, b) ->
    f_inner = (a, b) ->
        if gt a b then
          a_square \ # `\` is the line continuation character
            = a * a
          a_square # return
        else
            b * b # return
    ;
    f_inner a b
;

print_test = () -> print "test" ;

# Recursive functions
fact = (n) -> if eq n 0 then 1 else n * fact (n - 1) ;

# Higher order functions
apply: ((T) -> U, T) -> U = (f, x) -> f x ;

# Partial application
add: (T, T) -> T = (a, b) -> a + b ;
add_1: (int) -> int = add 1 ;

# Function call
result = add 1 2

# Function Composition
f1 = (x) -> x + 1
f2 = (x) -> x * 2
f3 = f3 = f1 (f2 x) # f1 . f2
result = f3 1 # 3
```
# Lists Overview

```python
l: [int] = [1, 2, 3, 4, 5] # mutable with cons and concat
head = hd l # 1
tail = tl l # [2, 3, 4, 5]
head, tail = l
first, second, tail = l
new_list = 0 : l # [0, 1, 2, 3, 4, 5] cons operator
new_list2 = l ++ [6, 7, 8, 9] # [1, 2, 3, 4, 5, 6, 7, 8, 9] concat operator
match_list: [int] -> str = (l) ->
  match l
  | [] => "empty"
  | [single] => match single
                | 0 => "zero"
                | _ => "other"
                ;
  | [first, second] => "two"
  | first : second : tail => "first, second and tail"
  | head : tail => "head and tail"
  ;
;

# Some List Functions
map: ((T) -> U, [T]) -> [U] = (f, l) ->
    match l
    | [] => []
    | head : tail => f head : map f tail
    ;
;

filter: ((T) -> Bool, [T]) -> [T] = (f, l) ->
    match l
    | [] => []
    | head : tail =>
        if f head then
            head : filter f tail
        else
            filter f tail
    ;
;
```

# Tuples Overview

```python
tuple: (int, str, int, str) = (1, "a", 2, "b") # immutable
first = tuple.0
second = tuple.1
third = tuple.2
fourth = tuple.3
(one, a, two, b) = tuple
```
