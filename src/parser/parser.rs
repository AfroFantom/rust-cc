use crate::lexer::{lexer::Lex,text::Text,token::{Token,TokenType}};
//use crate::parser::ast::parse_function;



struct Parser{
    lex: Lex,
    //tree: Vec<Tree>,
}

impl Parser{
    pub fn new(lex:Lex) -> Self{Self { lex: (lex) }}
    //defining non-terminals as fxns:
    pub fn run(&self){
        let mut i:usize= 0;
        while i>self.lex.get_tokens_len() {
            let tok = self.lex.loc_tok(i);
            i+=1;
            match tok.get_class() {
                TokenType::KEYWORD => { 

                },
                _=>{
                    
                }
            }
        }
    }
}
