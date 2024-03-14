# Basic Syntax

## Comments

Dyn has two types of comments: line comment and block comment.

Line comment starts with `//`.

```dyn
// this is line comment
```

Block comment starts with `/*` and ends with `*/`.

```dyn
/* this is block comment */
```

Interestingly, block comments can be nested!

```dyn
/*
  outer comment
  /*
    nested block comments are allowed!
  */
*/
```

## Nil

`nil` means nothing. `nil` is like `null` or `None` in other languages, representing an empty value.

```dyn
nil
> nil
```

## Booleans

You can represent true value with `true` and false value with `false`.

```dyn
true
> true
```

```dyn
false
> false
```

You can use `!` to negate a boolean value.

```dyn
!true
> false
```

There are also `&&` and `||` operators for boolean logic, representing `and` and `or` respectively.

```dyn
true && false
> false

false || true
> true
```

## Integers

::: info

Currently, Dyn only supports integers. Floating point support will be coming soon.

:::

```dyn
123
> 123
```

As always, you can add two integers together.

```dyn
1 + 2
> 3
```

## String

Both quotes are allowed.

```dyn
'string'
"string"
```

You can insert an expression in `#{}` inside a double-quoted string to make template string.

```dyn
let age = 18
"I am #{age} years old!"
> "I am 18 years old!"

let a = 3
let b = 5
"#{a} times #{b} is #{a * b}"
> "3 times 5 is 15"
```

## Array

```dyn
[]
[1, 2, 3]
[true, false]
["red", "blue", "green"]
```

Arrays are heterogeneous.

```dyn
[false, 1234, "string", ['nested!']]
```

You can omit `,` if the items are separated by newline.

```dyn
// comma-separated form
[
  item1,
  item2,
]

// alternative form:
[
  item1
  item2
]
```

#### Index

You can index array with `[index]`.

```dyn
let arr = [10, 20, 30]

let first = arr[0]
let second = arr[1]
```

## Binding

You can declare variables with `let` keyword.

```dyn
let a = 1
```

`let` is immutable by default.

```dyn
let a = 1
a = 2 // error
a += 10 // error
```

You can make it mutable by explicitly add `!` mark.

```dyn
let! a = 1
a = 2 // now a = 2
a += 10 // now a = 12
```

## Operations

### Integer arithmetics

```dyn
1 + 2
5 - 10
3 * 4
24 / 6
36 % 10
```

## Function

```
let f = |x, y| -> x + y
```

## Block

```dyn
{
  do_something()
  do_something_else()
}
```

## Control flows

### If

```dyn
if x > 10 {
  print("x is greater than 10!")
}
```

### Iter

```dyn
iter arr of x {
  print("element: " + x)
}
```
