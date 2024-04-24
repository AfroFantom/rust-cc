use crate::lexer::token::{Token};
use crate::lexer::lexer::Lex;
use std::{cell::RefCell, rc::Rc};
/*

Function{
    name: string
    statement:stmt
}

stmt{
    return:return
      or
    assign:assign
}

assign{
    type: string.tokentype
    identifier: string.tokenidentifier
    literal:string.tokenliteral
    val:const
}

return{
    expression:const
}

const{
    const_val:i32
    fn convert_literal
}

*/


pub struct Function{
    name:String,
    statement:Box<Statement>,
}

pub struct Statement{
    ret: Box<Return>,
    assign: Box<Assign>
}

pub struct Assign{
    a_type:String,
    idtfr:String,
    literal:String,
    val: Box<Const_int>,
}

pub struct Return{
    expr:Box<Const_int>
}

pub fn set_Return_const_int(i:i32) -> Return{
        let c = Const_int{
            const_val: i,
        };

        let expr= Box::<Const_int>::new(c);
        Return{expr:(expr)} 
}


pub struct Const_int{const_val:i32}