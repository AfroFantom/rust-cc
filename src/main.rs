mod lib {
    pub mod lexer;
    pub mod text;
    pub mod token;
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