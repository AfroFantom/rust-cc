use     crate::lib::text::Text;
use     crate::lib::token::{Token,TokenType};


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
        "struct","if","else",]);
    if keywords.contains(&literal.as_str()){
        return Token::new(TokenType::KEYWORD,literal.to_string(), start, end, line);
    }else{
        return Token::new(TokenType::IDENTIFIER,literal.to_string(),start,end,line);
    } 
}


pub struct Lex{
    text: Text,
    tokens: Vec<Token>,
}

impl Lex{
    pub fn new(text:Text) -> Self {
        Self {
            text: (text),
            tokens: (Vec::new())
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
                    self.tokens.push(
                        string_literal_tokeniser(&buf.clone(), start, self.text.get_offset(), self.text.get_line())
                    );
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
                    self.tokens.push(
                        numeric_literal_tokeniser(buf.clone(), start, self.text.get_offset(), self.text.get_line())
                    );
                    buf.clear();
                }

                let ch = self.text.loc();
                match ch{
                    //' ' => {println!("lex.run() found a whitespace pass:{}",pass)},
                    ')' =>{
                        let tok = Token::new(
                            TokenType::OPENBRACE,
                            ")".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line());
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '(' => {
                        let tok = Token::new(
                            TokenType::CLOSEBRACE,
                            "(".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line());
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '{' => {
                        let tok = Token::new(
                            TokenType::OPENPARENTHESIS,
                            "{".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line());
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '}' => {
                        let tok = Token::new(
                            TokenType::CLOSEPARENTHESIS,
                            "}".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line());
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    ';' => {
                        let tok = Token::new(
                            TokenType::SEMICOLON,
                            ";".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                                self.text.get_line());
                                //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                                self.tokens.push(tok);
                    },
                    ',' => {
                        let tok = Token::new(
                            TokenType::COMMA,
                            ",".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line()
                            );
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '>' => {
                        let tok = Token::new(
                            TokenType::GREATERTHAN,
                            ">".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line()
                            );
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '<' => {
                        let tok = Token::new(
                            TokenType::LESSTHAN,
                            "<".to_string(),
                            self.text.get_offset(),
                            self.text.get_offset(),
                            self.text.get_line()
                            );
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    _ => println!("not a valid match pass:{}",pass),
                }
                
                //println!("lex.run: {} char: {}",pass,ch1);
                
                self.text.advance();
                pass+=1;
           }
        }
        self.tokens.push(Token::new(TokenType::EOF, "EOF".to_string(), self.text.get_offset(), self.text.get_offset(), self.text.get_line()))
    }

    pub fn print(&self){
        for token in & self.tokens{
            let string;
            match token.get_class() {
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
            };
            println!("ltrl:     {}  token:      {}",token.get_literal(),string);
        }
    }

}


