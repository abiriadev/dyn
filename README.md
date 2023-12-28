<h1 align="center">dyn</h1>
<p align="center">Highly dynamic and powerful scripting language</p>

## Example

```dyn
print("Hello, world!")
```

## Syntax Overview

### Comments

```dyn
// single line comment

/* block comment */
```

### Arithmetics

You can perform basic arithmetics like addition, subtraction, multiplication, etc.

```dyn
1 + 2
5 - 10
3 * 4
50 / 7
36 % 5
```

### Variable declaration

Use `let` keyword to declare an immutable variable.

```dyn
let x = 5
let y = x + 10
print(y) // 15
```

You can add `!` to make the variable mutable.

```dyn
let! a = 0
a = 5
print(a) // 5
a += 2
print(a) // 7
```

### Conditional statements

Use `if` expression to execute a block based on conditions.

```dyn
let x = 10
if x > 5 {
	print("x is greater than 5")
}
```

You can use `else` to handle the other case.

```dyn
let today = "Monday"
if today == "Saturday" || today == "Sunday" {
	print("It's weekend!")
} else {
    print("It's weekday :(")
}
```

### Loop

Use `iter` and `of` keywords to iterate over an array.

```dyn
let friends = [
	"abiria"
	"andjsrk"
	"kangjun"
	"ensuta"
]

iter friends of person {
	print("Hello, " + person + "!")
}
```

## Build

```sh
$ git clone https://github.com/abiriadev/dyn && cd dyn
$ cargo install --path ./crates/cli
```

### Run REPL

```sh
$ dyn
# if you want to execute a script, pass the file path as an argument
$ dyn ./examples/hello.dyn
```
