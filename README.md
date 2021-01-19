## AST Parser for JSON

Using bottom-up recursive approach to avoid using RefCell, Rc, unsafe, etc, 

ToDo: Files containing new lines will be parsed correctly but the span of nodes will be wrong, has to build that feature.

Example:
```
JSON: {"a":5,"b":[4,5 , "gf"]}

Parsed Tree:
[ { type: "Object", children: 
    [ { type: "Property", 
        key:  
            { type: "Literal", value: "a\""  },
        value:  
            { type: "Literal", value: 5  }  
      },  
      { type: "Property",
        key:  
            { type: "Literal", value: "b\""  },
        value: 
            { type: "Array", 
                children: 
                    [ { type: "Literal", value: 4  },  { type: "Literal", value: 5  },  { type: "Literal", value: "gf\""  } ]  
            }  
       } 
    ]  
  } 
]

```
