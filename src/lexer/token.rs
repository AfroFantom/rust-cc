pub enum TokenType{
    OPENBRACE,
    CLOSEBRACE,
    OPENPARENTHESIS,
    CLOSEPARENTHESIS,
    SEMICOLON,
    GREATERTHAN,
    LESSTHAN,
    IF,
    ELSE,
    COMMA,
    KEYWORD,
    ASSIGN,
    IDENTIFIER,
    NIL,
    NEWLINE,
    WHITESPACE,
    INTLITERAL,
    EOF,
}
pub struct Token{
    class: TokenType,
    literal: String,
    start: usize,
    end: usize,
    line:usize,
}


impl Token{
    pub fn new(ttype:TokenType, literal:String,start:usize,end:usize,line:usize)->Self{
        Self { 
            class: (ttype), 
            literal: (literal), 
            start: (start), 
            end: (end), 
            line: (line)
        }
    }
    

    pub fn get_class(&self) -> &TokenType{&self.class}

    pub fn get_literal(&self) -> &String{&self.literal}

}
