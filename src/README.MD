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
- Code Generation
