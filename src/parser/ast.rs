use crate::lexer::token::{Token};
use crate::lexer::lexer::Lex;
use std::{cell::RefCell, rc::Rc};

pub enum Prg{
    Fnxn(Box<Fxn>),
}

pub enum Fxn{
    Name(String),
    Stmnt(Box<Stmt>)
}


pub enum Stmt{
    Ret(Box<Expr>),
}


pub enum Expr{
    Ident(String),
    Lit(Box<Type>),
}

pub enum Type{
    Int(i32),
    //Float(f32),
    //Str(String),

}