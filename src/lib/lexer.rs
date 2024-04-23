use std::{collections::HashSet, env, fs::File, io::{BufRead, BufReader}};

enum TokenType{
    OPENBRACE,
    CLOSEBRACE,
    OPENPARENTHESIS,
    CLOSEPARENTHESIS,
    SEMICOLON,
    COMMA,
    KEYWORD,
    IDENTIFIER,
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

}

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
        "int",
        "float",
        "char",
        "return",
        "struct"
        ]);
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
                let start = self.text.offset;
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
                        string_literal_tokeniser(&buf.clone(), start, self.text.offset, self.text.line)
                    );
                    buf.clear();
                }
                let start = self.text.offset;
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
                        numeric_literal_tokeniser(buf.clone(), start, self.text.offset, self.text.line)
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
                            self.text.offset,
                            self.text.offset,
                            self.text.line);
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '(' => {
                        let tok = Token::new(
                            TokenType::CLOSEBRACE,
                            "(".to_string(),
                            self.text.offset,
                            self.text.offset,
                            self.text.line);
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '{' => {
                        let tok = Token::new(
                            TokenType::OPENPARENTHESIS,
                            "{".to_string(),
                            self.text.offset,
                            self.text.offset,
                            self.text.line);
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    '}' => {
                        let tok = Token::new(
                            TokenType::CLOSEPARENTHESIS,
                            "}".to_string(),
                            self.text.offset,
                            self.text.offset,
                            self.text.line);
                            //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                            self.tokens.push(tok);
                    },
                    ';' => {
                        let tok = Token::new(
                            TokenType::SEMICOLON,
                            ";".to_string(),
                            self.text.offset,
                            self.text.offset,
                                self.text.line);
                                //println!("lex.run() found grammar-literal:{} pass: {}",ch,pass);
                                self.tokens.push(tok);
                    },
                    ',' => {
                        let tok = Token::new(
                            TokenType::COMMA,
                            ",".to_string(),
                            self.text.offset,
                            self.text.offset,
                            self.text.line
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
        self.tokens.push(Token::new(TokenType::EOF, "EOF".to_string(), self.text.offset, self.text.offset, self.text.line))
    }

    pub fn print(&self){
        for token in & self.tokens{
            let mut string;
            match token.class {
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
            };
            println!("ltrl:     {}  token:      {}",token.literal,string);
        }
    }

}


pub struct Text{
    source: Vec<String>,
    line:usize,
    offset:usize,
}

impl Text{
    pub fn new()->Self{
        let source:Vec<String> = Vec::new();
        Self { source: (source), 
            line: (0),
            offset: (0)
         }
    }


 //   pub fn len()
    pub fn is_end(&self) -> bool {
        self.line >= self.source.len()
    }


    pub fn read_source(&mut self,filename:&str){
        let f = File::open(filename).expect("File not opening");
        let mut f = BufReader::new(f);
        let mut output:Vec<String> = Vec::new();
        let mut idx:usize = 0;
        let mut buf:String  = String::new();
        while let Ok(n) = f.read_line(&mut buf){
            //EOF
            if n==0{
                buf.push('\0');
                break;
            }
            let temp:String= buf.clone();
            //println!("text read_source idx: {} temp/buf:{}",idx,temp);
            idx+=1;
            output.push(temp);
            buf.clear();
        }
        self.source = output;
    }

    pub fn advance(&mut self){
        if self.offset < self.source[self.line].len() {
            self.offset+=1;
        }
        else {
            self.offset = 0;
            self.line += 1;
        }
    }

    pub fn loc(&self) -> char{
        let s = &self.source[self.line];
        let ch =s.chars()
            .nth(self.offset);
        match ch {
            Some(ch) => return ch,
            None => {
                let ch ='\0'; 
                return ch
            },
        } 
       
       
    }
    
    pub fn peek(&self)-> char{
        let s=&self.source[self.line];
        let ch = s.chars()
        .nth(self.offset + 1 as usize);
        match ch {
            Some(ch) => return ch,
            None => {
                let ch ='\0'; 
                return ch
            },
        }
    }
 }