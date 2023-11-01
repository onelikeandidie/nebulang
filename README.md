# Nebulang

Another scripting language, focused on input and graphics for vulkan. The 
runtime is written in Rust. The language is transpiled to Nebulang ByteCode and
ran by the Nebulang runtime.

## Syntax Example

Nebulang syntax tries to spare more keywords for whatever you want to write,
keywords like `function`, `return`, `export` are replaced with symbols like
`function` is `#` and `return`/`export` are `<<`. The reserved words are also
typically smaller, e.g. `float` is `flt`, most of them being 3 characters long.

```nebulang
// This is a comment

// This is a function declaration
#add (int a, int b) int {
    << a + b; // Return is typed <<
}

// Functions can also be declared in a short syntax for 1-liners
#subtract (int a, int b) int << a - b;

// Programs always start from the #main function. This function will have the
// arguments passed from the call from the terminal
#main (vec<str> args) int {
    int a = 1;
    int b = 2 // Semi-colon is optional
    int c = add(a, b);
    print("{}", c); // Rust-like  🦀
    << 0;
}
```

## Control Flow

`If` statements are simple, they have syntax similar to javascript but without
the parentisis for the condition. To declare an if statement, you can use the
question mark symbol, followed by the condition and a scope.

```nebulang
#main (vec<str> args) int {
    int a = 1;
    int b = 2;
    ? a != b {
        a = b;
    }
    ? a == b {
        print("Values are equal!");
    }
    << 0
}
```

Loops are very simple, only 1 type of loop is available, equivalent to 
while(true) in javascript. This loop needs to be broken to exit. Loops are
declared using the exclamation mark

```nebulang
#main (vec<str> args) int {
    int index = 0;
    int range = 10;
    ! {
        ? index >= range {
            << ! // This is used to break a loop
        } 
        print(index);
        index++;
    }
    << 0
}
```

## Operands

```nebulang
#main (vec<str> args) int {
    int a = 2 * 3; // Multiplication
    int b = 1 + 1; // Addition
    int c = 1 - 1; // Subtraction
    flt d = 2 / 2; // Division
    int e = 5 % 2; // Modulo - Remainder of division
    bol f = 1 == 1 // Equal check
    bol g = 1 >= 1 // Bigger or Equal check
    bol h = 1 <= 1 // Smaller or Equal check
    bol i = 1 > 1; // Bigger check
    bol j = 1 < 1; // Smaller check
    int k = 1 << 1 // Bit shift left
    int l = 2 >> 1 // Bit shift right
    int m = 1 | 3; // Bitwise OR
    int n = 1 & 2; // Bitwise AND
    << 0
}
```

## Inline comments

The only available commenting is to prefix your comment with "//". This will
make the intepreter skip over any thing until the end of the line. If you want
to comment inline however, you can use a semi-colon ";" to end your comment
prematurely.

```nebulang
#main (vec<str> args) int {
    str text = "The quick brown fox jumps over the lazy dog";
    int i = 0; //index of in the text
    ! {
        print(text[i] //print each char; );
        ? i >= text.length {
            << !
        }
        i++;
    }
    << 0;
}
```

## Importing other files

Files can export functions and variables.

```nebulang
// main.nl
#multiply@dep.nl

#main (vec<str> args) int {
    flt a = 2;
    flt b = 3;
    flt c = multiply(a, b);
    print("{}", c);
    << 0;
}

// dep.nl
// Exporting from a file is just like returning from a function!
<< #multiply (flt a, flt b) flt {
    << a * b;
}
```

## Variable Type Casting

Nebulang supports variable type casting.

```nebulang
#main (vec<str> args) int {
    int a = 1;
    flt b = 1;
    flt c = a~flt + b;
    print("{}", c);
    << 0;
}
```

## Arrays and Structures

Arrays are called Vectors `vec` and support int indexing. Structures are called
Maps `map`.

To specify the type of values in these lists, the `<type>` can must be used.
Example shown bellow.

```nebulang
#main (vec<str> args) int {
    vec<str> list = ["Hello", "World"];
    str name = "John";
    list = list.push(name);
    print("{}", list.join(" "));
    // This would print "Hello World John"
    << 0;
}
```

```nebulang
// Import the map struct from the std library
// the ^ symbol defines a struct (see further bellow)
^map@std

#main (vec<str> args) int {
    map<str><str> info = [
        "name","Nebulang"
        "type","scripted"
    ];
    print("{} {}", info.name, info.type);
    // This should print "Nebulang scripted"
    << 0
}
```

## Mutability

All data types are immutable by default. Variables can be reassigned with their
variable name without having to declare the type again.

## Data Types

| Keyword | Data Type |
|------|------|
| int | integer / whole number |
| flt | float / fixed point precision number |
| bol | boolean / true or false |
| chr | char / single character |
| str | string / array of characters |
| vec | array / list of values |

### String delimiters

Sometimes you want to print or save a string that includes double or single 
quotes. You could use 'double "quoting"' but Nebulang supports quoting 
operators for you to set your own delimiter (temporarily or globaly).

```nebulang
#main (vec<str> args) int {
    // The symbol after "qq" is the temporary delimiter
    str out = qq*Hello "World"*;
    print(out);
    << 0
}
```

Globally can be set with the "--qq delimiter" flag on compilation or in the
config file. This will make the delimiter available to the whole source code
but you can still use single and double quotes.

## Defining your own data types (structs)

Think of structs like defined data structures on which you can call functions
attached to them.

```nebulang
^person {
    int id;
    str name;
}

// Creating a function for a struct can be declared like the following
#nameplate^person (self self) str {
    << format("{}:{}", self.id, self.name)
}

#main (vec<str> args) int {
    person employee = {
        id: 69,
        name: "John"
    }
    print("Personal ID: {}", employee.id);
    // Prints out "Personal ID: 69"
    print("Nameplate: {}", employee.nameplate());
    // Prints out "Nameplate 69:John" 
    << 0
}
```

### Extending a struct that already exists

Sometimes you want to keep functionality of an existing struct like `vec`. This
is also so that type checks pass in function calls.

```nebulang
// Create struct
^conveyor<t> {
    int max;
    vec<t> items;
}
// Add new function to struct
#new^conveyor<t> (int max, vec<t> items) self {
    << conveyor<t> {
        max: max,
        items: items,
    }
}
// Add the push, length, and indexing functions available for conveyor
^vec<t>^conveyor<t>;
// Override the push function
#push^conveyor<t> (self self, t new_item) self {
    vec<t> new_items = self.items;
    ? conveyor.length > self.max {
        new_items = new_items.unshift();
    }
    new_items = new_items.push(new_item);
    << conveyor
}
// Override the indexing function
#get^conveyor<t> (self self, int index) t {
    << self.items[index]
}
// Example program
#main (vec<str> args) int {
    conveyor<str> todo = conveyor:new(2, [
        "plant tree",
        "pet crab"
    ]~vec<str>);
    todo = todo.push("take vitamins");
    print("{},{}", todo[0], todo[1]);
    // Should print out "pet crab, take vitamins"
    << 0
}
```

## Nebulang Runtime

The nebulang runtime is written in Rust. However it doesn't require Rust to be
installed to run ByteCode.

### AST

First step is to parse the source code and turn it into less human readable
instructions for the computer. Each of these are called a node and they can 
be parents of other nodes. Each node will also have a pointer to their parent
node (which I left out for brevety) with whom they will check for expectations.

```nebulang
#main (vec<str> args) int {
    // This is a comment
    print("Hello World");
    << 0
}
```

Is interpreted as:

```json
{
    type: Root,
    nodes: [{
        type: Function("main")
        nodes: [{
            type: Params
            nodes: [{
                type: Type("vec"),
                nodes: [{
                    type: Type("str"),
                }]
            }, {
                type: Identifier("args"),
            }]
        }, {
            type: Type("int"),
        }, {
            type: Body,
            nodes: [{
                type: Comment("This is a comment"),
            }, {
                type: Call("print"),
                nodes: [{
                    type: Params,
                    nodes: [{
                        type: LiteralString("Hello World"),
                    }]
                }]
            }, {
                type: Return,
                nodes: [{
                    type: LiteralInt(0),
                }]
            }]
        }]
    }]
}
```