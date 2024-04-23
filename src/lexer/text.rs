use std::fs::File;
use std::io::{BufRead,BufReader};

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

    pub fn get_offset(&self) -> usize {self.offset}

    pub fn get_line(&self) -> usize {self.line}

    pub fn is_end(&self) -> bool {
        self.line >= self.source.len()
    }


    pub fn read_source(&mut self,filename:&str){
        let f = File::open(filename).expect("File not opening");
        let mut f = BufReader::new(f);
        let mut output:Vec<String> = Vec::new();
        let mut buf:String  = String::new();
        while let Ok(n) = f.read_line(&mut buf){
            //EOF
            if n==0{
                buf.push('\0');
                break;
            }
            let temp:String= buf.clone();
            //println!("text read_source idx: {} temp/buf:{}",idx,temp);
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