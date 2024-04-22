- Lexer 
    - Token 
        - enum 
            - already defined
            - advance thru each char in the source identifing lexemes and collecting them
            - send lexemes to tokeniser function 
            - tokeniser function collects tokens 
            - lookup how to pretty print 
            - could use offset and line combinations stored in a hashmap 
        -class
            - {
                type TokenEnum
                Literal String
                start usize
                end usize
              }
    - Text Class
        {
            source Vec [ string ] 
            line usize
            offset usize
        }
        - impl
            - new(filename)
                takes the file name and enters the files contents into
                a buffer that passes it into a bufreader instance that 
                creates an iterator of lines that can be put into source 
                sets line and offset to 0 
            - advance_char(self)-> char ch:
                takes self as arg and advances offset by 1 
                if it is the size of the line then set it to 0 and increment line
                (returns char value that can be used for match ?) 
            - peek(self):
                return the char value at offset+1
    - Lex Class
        {
            text Text
            tokens Token
        }
            - impl
                - new (text)
                    takes a text object and instantiates the text field with that sets tokens to an empty list 
                
                - run(self)
                    runs the lexer loop on the associated text field, calls tokeniser every time new character is introduced
                    tokeniser runs, returns control and the loop continues until text is over
                    add an EOF token at the end to finish the file, collect each token object and return the collection

                - tokeniser()
                    start with a char val in the text field and keep reading the next value until whitespace is encountered
                    keep continuing until new character found. each lexeme from this loop as it 
                    comes would be matched with the type from the token enum, from these we can return the tokens that we get 
                    return token and control to caller function
                        - check if ch is alphanumeric or not
                            if yes
                                if number then int literal
                                if letter keep going until whitespace 
                                    if lexeme in reserved map then keyword 
                                    else identifier
                            else
                                if not then assign valid punctuation mark 
                            return token object 


- Parser
    -To transform a list of tokens into an AST, we’ll use a technique called recursive descent parsing. We’ll define a function to parse each non-terminal symbol in the grammar and return a corresponding AST node. The function to parse symbol S should remove tokens from the start of the list until it reaches a valid derivation of S. If, before it’s done parsing, it hits a token that isn’t in the production rule for S, it should fail. If the rule for S contains other non-terminals, it should call other functions to parse them.

    Here’s the pseudocode for parsing a statement:
'''
    def parse_statement(tokens):
        tok = tokens.next()
        if tok.type != "RETURN_KEYWORD":
        fail()
            tok = tokens.next()
        if tok.type != "INT"
          fail()
        exp = parse_exp(tokens) //parse_exp will pop off more tokens
        statement = Return(exp)
        tok = tokens.next()
        if tok.type != "SEMICOLON":
            fail()
        return statement
    '''
Later, the production rules will be recursive (e.g. an arithmetic expression can contain other expressions), which means the parsing functions will be recursive too - hence the name recursive descent parser.
Task:

Write a parse function that accepts a list of tokens and returns an AST, rooted at a Program node. The function should build the correct AST for all valid stage 1 examples, and raise an error on all invalid stage 1 examples. If you want, you can also have your parser fail gracefully if it encounters integers above your system’s INT_MAX.

There are a lot of ways to represent an AST in code - each type of node could be its own class or its own datatype, depending on what language you’re writing your compiler in. For example, here’s how you might define AST nodes as OCaml datatypes:
- Code Generation
