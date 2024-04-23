use std::env;
use std::collections::{HashSet};
use std::error::Error;
use std::io::{self,BufReader,BufRead};
use std::fs::File;
use regex;

mod lib {
    pub mod lexer;
}
use lib::lexer::Lex;




//fn print(text:&Text){
//    let mut idx =0 ;
//    for line in text.source.iter(){
//        println!("print.util idx: {} line: {}",idx,line);
//    }
//}



fn main(){
    //let _= print(&txt);
    let mut lexer:Lex = Lex::seed();
    lexer.run();
    lexer.print();
}