use crate::lexer::text::Text;
use crate::lexer::token::{Token,TokenType};


use std::collections::VecDeque;
use std::{collections::HashSet, env};


    
// assign numeric literals their tokens 
    //TODO: add functionality for float literals as well 
pub fn numeric_literal_tokeniser(literal:String,start:usize,end:usize,line:usize) -> Token{ 
    Token::new(TokenType::INTLITERAL, literal, start, end, line)
}
    
// assign sting literals theirs tokens and return them 
// ascertain if the literal is a keyword or an identifier 
//keywords would in a reserved set capable of instant lookup 

pub fn string_literal_tokeniser(literal:&String,start:usize,end:usize,line:usize) -> Token{ 
    let keywords = HashSet::from([
        "int","float","char","return",
        "struct",]);
    if keywords.contains(&literal.as_str()){
        return Token::new(TokenType::KEYWORD,literal.to_string(), start, end, line);
    }else{
        return Token::new(TokenType::IDENTIFIER,literal.to_string(),start,end,line);
    } 
}


//assign the tokentype enum variant for the given char ch variable
pub fn char_literal_tokeniser(ch:char) -> TokenType{
    match ch{
        //' ' => {println!("lex.run() found a whitespace pass:{}",pass)},
        ')' => return TokenType::CLOSEBRACE,
        '(' => return TokenType::OPENBRACE,
        '{' => return TokenType::OPENPARENTHESIS,
        '}' => return TokenType::CLOSEPARENTHESIS,
        ';' => return TokenType::SEMICOLON,
        ',' => return TokenType::COMMA,
        '>' => return TokenType::GREATERTHAN,
        '<' => return TokenType::LESSTHAN,
        '=' => return TokenType::ASSIGN,
        _ => return TokenType::NIL,
    }
}

#[derive(Debug,Clone)]

pub struct Lex{
    text: Text,
    tokens: VecDeque<Token>,
    //i:usize,
}

impl Lex{
     //lexer util functions
    //pub fn loc_tok(self,i:usize) ->Token{
    //    self.tokens.get(i).unwrap()
    //}

    pub fn get_tok_class_cmp(&mut self, ttype: TokenType) -> bool {self.get_tok().get_class() == &ttype}
    
    //get_tok consumes tokens and consumes them again if the current one is nil 

    pub fn get_tok(&mut self) -> Token{
        let mut tok = self.tokens.pop_front().unwrap();
        while tok.get_class() == &TokenType::NIL{
            tok = self.tokens.pop_front().unwrap();    
        }
        tok
    }

    pub fn peek(&mut self) -> &Token{
        let mut tokens = self.tokens
            .iter()
            .peekable();
        tokens.peek().unwrap()
    }
                                                                                            
    pub fn is_tokens_empty(&self) -> bool{self.tokens.len() == 0}

    pub fn get_tokens_len(&self) -> usize { 
        self.tokens.len() 
    }

    pub fn new(text:Text) -> Self {
        Self {
            text: (text),
            tokens: (VecDeque::new()),
         }
    }
    pub fn seed()->Lex{
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        let mut txt:Text = Text::new();
        let _= txt.read_source(&filename);
        //let _= print(&txt);
        let lex = Lex::new(txt);
        lex
    }
    pub fn run(&mut self) {
        
        //runs the lexer loop on the associated text field, calls tokeniser every time new character is introduced
        //tokeniser runs, returns control and the loop continues until text is over
        //add an EOF token at the end to finish the file, collect each token object and return the collection
        let mut pass=0;
        while !self.text.is_end(){
            let ch1 = self.text.loc();
            if ch1 == '\n' {       
                //println!("lex.run found a newline: {}",pass);
                pass+=1;
                self.text.advance();
            }
            else {  
                let mut buf:String=String::new();  
                //Check for identifiers and keywords and assign tokens as needed
                let start = self.text.get_offset();
                while (self.text.peek().is_alphabetic()==true) 
                                ||
                      (self.text.loc().is_alphabetic()==true){
                    let ch= self.text.loc();
                    if ch == ' '{break;}
                    buf.push(ch);
                    self.text.advance();
                    pass+=1;
                }
                if !buf.is_empty(){
                    //println!("lex run found string-literal: {} pass: {}",buf,pass);
                    match buf.as_str(){
                        "if" => {
                            let tok = Token::new(
                                TokenType::IF,
                                "if".to_string(),
                                start,
                                self.text.get_offset(),
                                self.text.get_line(),

                            );
                            self.tokens.push_back(tok)
                        }
                        "else" => {
                            let tok = Token::new(
                                TokenType::ELSE,
                                "else".to_string(),
                                start,
                                self.text.get_offset(),
                                self.text.get_line(),

                            );
                            self.tokens.push_back(tok)
                        }
                        _ => {self.tokens.push_back(
                                    string_literal_tokeniser(&buf.clone(), 
                                                            start, 
                                                            self.text.get_offset(), 
                                                            self.text.get_line())
                            )
                        }
                    }
                    buf.clear();
                }
                let start = self.text.get_offset();
                //check for integer literals and assign tokens
                while (self.text.peek().is_numeric()==true) 
                                ||
                      (self.text.loc().is_numeric()==true){
                    let ch= self.text.loc();
                    if ch == ' '{break;}
                    buf.push(ch);
                    self.text.advance();
                    pass+=1;
                }
                if !buf.is_empty(){
                    //println!("lex run found numeric-literal: {} pass: {}",buf,pass);
                    self.tokens.push_back(
                        numeric_literal_tokeniser(buf.clone()
                                                , start
                                                , self.text.get_offset()
                                                , self.text.get_line())
                    );
                    buf.clear();
                }

                let start= self.text.get_offset();
                let ch = self.text.loc();
                let toktype = char_literal_tokeniser(ch); 
                let tok= Token::new(
                    toktype
                    , ch.to_string()
                    ,start
                    ,self.text.get_offset()
                    , self.text.get_line()
                );
                self.tokens.push_back(tok);
                //println!("lex.run: {} char: {}",pass,ch1);
                
                self.text.advance();
                pass+=1;
           }
        }
        self.tokens.push_back(Token::new(TokenType::EOF
                            , "EOF".to_string()
                            , self.text.get_offset()
                            , self.text.get_offset()
                            , self.text.get_line()))
    }

    pub fn token_print(token:Token) -> &'static str{
            let string;
            match token.clone().get_class() {
                TokenType::OPENBRACE => string="OPENBRACE",
                TokenType::CLOSEBRACE => string="CLOSEBRACE",
                TokenType::OPENPARENTHESIS => string="OPENPARENTHESIS",
                TokenType::CLOSEPARENTHESIS => string="CLOSEPARENTHESIS",
                TokenType::SEMICOLON => string="SEMICOLON",
                TokenType::COMMA => string="COMMA",
                TokenType::KEYWORD => string="KEYWORD",
                TokenType::IDENTIFIER => string="IDENTIFIER",
                TokenType::INTLITERAL => string="INTLITERAL",
                TokenType::EOF => string="EOF",
                TokenType::GREATERTHAN => string=">",
                TokenType::LESSTHAN => string="<",
                TokenType::IF => string="IF",
                TokenType::ELSE => string="ELSE",
                TokenType::ASSIGN => string="=",
                TokenType::NIL => string="NIL",
            };
            //println!("ltrl:     {}  token:      {}",token.get_literal(),string);
            string
   }
}


