## Array

In Dyn, arrays are ordered collections of items.

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

// terse form:
[
  item1
  item2
]
```

### Indexing

You can index array with `array[index]`.

```dyn
let arr = ["first", "second", "third"]

arr[0]
> "first"

arr[1]
> "second"
```

Remember that array index starts at `0`.
