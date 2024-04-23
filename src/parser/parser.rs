use crate::lexer::{lexer::Lex,text::Text,token::{Token,TokenType}};

struct Parser{
    tokens: Vec<Token>,
    //tree: Vec<Tree>,
}

impl Parser{
    pub fn new(tokens:Vec<Token>) -> Self{Self { tokens: (tokens) }}
    //defining non-terminals as fxns:

    pub fn run(){todo!()}
}
