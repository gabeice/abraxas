```ebnf
expression = atom | "(", { whitespace }, identifier, { whitespace }, [ { expression, whitespace + }, expression ], { whitespace }, ")" ;
atom = identifier | literal ;
literal = number | char | string | list ;
identifier = letter, { character - ( "(" | ")" | "[" | "]" | whitespace ) } ;
number = [ "-" ], digit +, { ".", digit + } ;
char = "'", character - "'", "'" ;
string = '"', { character - '"' }, '"' ;
list = "[", { whitespace }, [ { atom, whitespace + }, atom ], { whitespace }, "]";
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
whitespace = ? all whitespace characters ? ;
character = ? all characters ? ;
```

