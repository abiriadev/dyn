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
