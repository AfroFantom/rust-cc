#[derive(PartialEq,Clone,Debug)]
pub enum TokenType{
    ASSIGN,                 // =
    CLOSEBRACE,             // )
    CLOSEPARENTHESIS,       // }
    COMMA,                  // ,
    ELSE,                   // else
    EOF,                    // EOF
    GREATERTHAN,            // <
    IDENTIFIER,             // ident
    IF,                     // if 
    INTLITERAL,             // int values 
    KEYWORD,                // reserved words 
    LESSTHAN,               // >
    NIL,                    // 
    OPENBRACE,              // (
    OPENPARENTHESIS,        // {
    SEMICOLON,              // ;
}


#[derive(Debug,Clone)]
pub struct Token{
    class: TokenType,
    literal: String,
    start: usize,
    end: usize,
    line:usize,
}

impl Eq for Token{}

impl PartialEq for Token{
    fn eq(&self, other:&Self) -> bool{
        self.class == other.class
    }
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
