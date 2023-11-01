# Rework Nebulang:

Abstract tree parser needs to be reworked into steps.

I think the first step is a symbol parser to separate symbols by spaces and 
some other requirements. For example:

```nebulang
#main (vec<str> args) int {
	//This is a comment
	print("Hello World");
	<< 0
}
```

This snippet needs to go through a couple of different steps before being
transformed into an abstract syntax tree. The symbol analyser should go through
and separate "#main" and "(" into 2 symbols while also attaching information of
where they are in the source file (for possible mappings and debugging).

So the first step would be to filter what is not "code" or real operations, 
what that means is that comments and other useless symbols should be filtered
out by the interpreter so that we can have a easier time compiling into opcodes

The expected result of symbol parsing this snippet would be:

``#main, (, vec, <, str , >, args, int, {, print, (, 'Hello World', ), ;, <<,
0, }``

Then the interpreter would take over from the symbols and generate the node 
tree. Something like so:

```json
{
    type: Root,
    nodes: [{
        type: Function,
        identifier: "main"
        nodes: [{
            type: Param,
            nodes: [{
                type: Type,
                identifier: "vec"
                nodes: [{
                    type: Type,
                    identifier: "str"
                }, {
                    type: Identifier,
                    identifier: "args"
                }]
            }]
        }, {
            type: Type,
            identifier: "int"
        }, {
            type: Body,
            nodes: [{
                type: Call,
                identifier: "print"
                nodes: [{
                    type: Param,
                    nodes: [{
                        type: LiteralString,
                        identifier: "Hello World"
                    }]
                }]
            }, {
                type: Return,
                nodes: [{
                    type: LiteralInt,
                    identifier: "0"
                }]
            }]
        }]
    }]
}
```

_Nodes ommitted for brevity._
