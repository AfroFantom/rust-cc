use crate::lexer::{lexer::Lex,text::Text,token::{Token,TokenType}};
use crate::parser::ast::{Prg,Fxn,Stmt,Type};



struct Parser{
    lex:Lex,
    //tree: Vec<Tree>,
}

impl Parser{
    pub fn new(lex:Lex) -> Self{
        Self { 
            lex:(lex)
    }}
    /*
    run:
        if terminal:
            return token and necessary value
        if non-terminal:
            call fxn
    */
    pub fn run(&mut self){
        self.parse_program();
    }

    pub fn parse_program(&mut self){
        self.parse_function();
    }

    pub fn parse_function(&mut self) -> Option<Fxn>{
        let tok = self.lex.get_tok();
        match tok.get_class() {
            TokenType::KEYWORD => {
                todo!()
            }  

            _ => {
                return None;
            }
        }
    }

    pub fn parse_statement(&mut self){
        todo!()
    }

    pub fn parse_expression(&mut self){
        todo!()
    }

}
