# PuppyScript

## Features

### Functions

```
fn foo(a: u32, b: u32) -> u32 {
    return a + b
}

let r = foo(a = 1, b = 2)

// or

let r2 = foo {
    a: 1
    b: 3
}
```

### Structures

```
struct Person {
    name: String
    age: u32
}
```

You can implement functions on structures
```
struct Person {
	name: String
	age: u32

	fn say_hello() {
		print("Hello ", this.name)
	}
}
```

### Arrays
```
strict Person {
	name: String
	friends: Person[]
}
```

### Anynomous objects

```
const obj = {
    name: "matti"
    age: 1234
}
```

### XML

XML syntax is supported out of the box.
```

let obj = <Person>
    <name>
        Matti
    </name>
    <age>
        1234
    </age>
</Person>

let view = <View
    <Text color="red">
        Hello world
    </Text>
</View>
```

It is possible to define XML objects with struct
```
struct Person {
    name: String
    age: u32
    children?: [Person]
}

const obj = <Person name="Teppo" age=53>
    <Person name="Matti" age=23>
    <Person name="Maija" age=21>
<Person>

const ui_component = <View style={ flexDirection: "column" }>
    <Text>
        Hello world
    </Text>
</View>
```

### JSON

JSON syntax is supported out of the box.
```
const obj = {
	name: "Matti"
	age: 1234
}
```


### Operators

```
1 + 1 == 2 // Addition
1 - 1 == 0 // Substraction
2 * 2 == 4 // Multiplication
2 / 2 == 0 // Division
8 % 5 == 3 // Modulo operation
```

### If

```
let a = 5
if a == 5 {

}
```

### For

```
const arr = [1, 2, 3, 4, 5]
for item in arr {

}
```

For with range
```
for i in 0..5 {

}
```

### While

```
let a = 0
while a < 5 {
    a++
}
```

### Switch

```
let a = 5
switch a {
	case 1 {
		// Do something
	}
	case 2 {
		// Do something
	}
	default {
		// Do something
	}
}
```

Works with enums
```
enum Color {
	Red
	Green
	Blue
}

let color = Color.Red

switch color {
	case Color.Red {
		// Do something
	}
	case Color.Green {
		// Do something
	}
	case Color.Blue {
		// Do something
	}
}
```

Forces to handle all cases
```
enum Color {
	Red
	Green
	Blue
}

let color = Color.Red

// this will cause compile error
switch color {
	case Color.Red {
		// Do something
	}
	case Color.Green {
		// Do something
	}
}
```

## Types

### Pritimitive types

Primitive types are defined either with emulation or with compiler instruction.

```
@primitive("u32")
struct u32
```

```
struct bfloat {
    // Implementation
}
```

### Type alias

```
type Animal = {
    make_sound: fn () -> void
}
```

## Concurrency

### Require locks automatically

```
import std

fn main() {
    let var = 1

    fn foo() {
       var += 1 
    }

    std.co(foo())
    std.co(foo())
    std.co(foo())
}
```

### Wait for coroutines

```
import std

fn foo() {
    return 1
}

fn main() {
    let c1 = std.co(foo())
    let c2 = std.co(foo())
    let [v1, v2] = std.join([c1, c2])
}
```

### Select one of the channels

```

enum CoResult {
	Done
	Pending
}

fn foo() {
    return 6
}

fn main() {
    let t1 = std.co(foo)
    let t2 = std.co(foo)

    std.select {
        t1: r => {

        }
        t2: r => {

        }
    }
}
```

### @pararell compiler instruction

Tell compiler that function migth be executed pararell. This could force depended code to handle synchronization.

```
fn thread(@pararell fn) {
    std.syscall(...)
}

```

### Iterators

```

fn foo() {
    return 5
}

fn main() {
    c1 = std.co(foo())
    c2 = std.co(foo())
    c3 = std.co(foo())

    let items = [c1, c2, c3]

    // Will wait coroutines in order they are defined
    for item in items {
        print(item)
    }

    // Will iterate co routines in order they complete
    for item in std.select(items) {
        print(item)
    }
}
```

### SIMD

```
import std.simmd

a = u32x4
b = u32x4
let c = a * b

```

## Metaprogamming

**Source code**

Metaprogramming on level of source code
```
fn process_source(source) {
    // Do some transformation with source
    return source
}

@source(process_source)
fn foo() {

}
```

**Tokens**

Metaprogramming on level of tokens
```
fn process_tokens(tokens) {
    // Do some transformation with tokens
    return tokens
}

@tokens(process_tokens)
fn foo() {

}
```

**AST**

Metaprogramming on level of AST
```
fn process_ast(ast) {
    // Do some transformation with ast
    return ast
}

@ast(process_ast)
fn foo() {

}
```

**IR**

Metaprogramming on level of IR
```
fn process_ir(ir) {
    // Do some transformation with ir
    return ir
}

@ir(process_ir)
fn foo() {

}
```

