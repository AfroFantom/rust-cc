# Rust_CC

### Lexer

- Lex Class imports text and token classes, instantiates the objects and
tokenises the lexemes in the source text. Lex Class instantiated with the filename served as arg(1) in the cli cargo call
Usage:

```markdown
cargo run -file 
```

- Input: Takes in filename as args[1]
- Output: Vector of tokens

### Parser

- Parser class that performs recursive descent on a vec! of tokens that is then used to ascertain the flow of the program, by making an ast of the source code 
- #### Parse() Notes 
    - Init a class of AST 
    - root at start of program (root -> Program)
    - children are the subcomponents of Program
    - break down the subcomponents further
    - recursively descend down this line of subcomponents
    - use rules to call the necessary functions
    - **Find way to translate rules, Draft accurate rules**
    - control keeps going down until we find terminal tokens
        - two types of tokens, they either call a function or return a ast node obj
        - each call to a function eventually will return an AST object 
    - Rules:
        ```markdown
            <program> ::= <function>
            <function> ::= "int" <id> "(" ")" "{"  <statement>  "}" 
            <statement> ::= "return" <exp> ";"
            <exp> ::= <int>
        ```
    - PseudoCode:
        ```markdown
            Parse(){

            }
        ```


### Code-Gen
- Code-gen class that gives us a generated assembly file after being paramterised with the AST of the associated source code