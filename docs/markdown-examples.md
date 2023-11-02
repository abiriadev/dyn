# Syntax

## Comments

```dyn
// this is line comment
```

```dyn
/* this is block comment */
```

## Literals

### Nil

`nil` is like `null` or `None` in other languages.

```dyn
nil
```

### Booleans

```dyn
true
```

```dyn
false
```

### Integer Literal

```dyn
1
```

### String Literal

Both quotes are allowed.

```dyn
'string'
"string"
```

### Array Literal

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

You can omit `,` if there is newline.

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
