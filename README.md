## AST Parser for JSON

Using bottom-up recursive approach to avoid using RefCell, Rc, unsafe, etc, 

Example:
```
JSON: {"a":5,"b":
[4,5 , "gf"]
}

Parsed Tree:
[ {
    type: "Object",
    children: [
         {
            type: "Property",
            key:  {
                type: "Literal",
                value: "a\"",
            },
            value:  {
                type: "Literal",
                value: 5,
            },
            loc:  {
                start:  {
                    line: 1,
                    column: 2,
                },
                end:  {
                    line: 1,
                    column: 7,
                },
            },
        },
         {
            type: "Property",
            key:  {
                type: "Literal",
                value: "b\"",
            },
            value:  {
                type: "Array",
                children: [
                     {
                        type: "Literal",
                        value: 4,
                    },
                     {
                        type: "Literal",
                        value: 5,
                    },
                     {
                        type: "Literal",
                        value: "gf\"",
                    },
                ],
            },
            loc:  {
                start:  {
                    line: 1,
                    column: 8,
                },
                end:  {
                    line: 2,
                    column: 12,
                },
            },
        },
    ],
    loc:  {
        start:  {
            line: 1,
            column: 2,
        },
        end:  {
            line: 3,
            column: 0,
        },
    },
}]

```
