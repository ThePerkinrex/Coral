use id_arena::{Arena, DefaultArenaBehavior};
use logos::Logos;

use crate::{
    fs::{File, FileId},
    lexer::Token,
};

mod ast;
mod error;
mod fs;
mod lexer;
mod parser;
mod span;

type FileArena = Arena<File>;

fn main() {
    println!("Hello, world!");
    let mut arena: Arena<File> = Arena::new();
    let f_a = arena.alloc(File {
        name: "a".into(),
        contents: "fn main(){}".into(),
    });
    let f_b = arena.alloc(File {
        name: "b".into(),
        contents: "fn main() {hello == b && a >= c; #a != 1 }".into(),
    });
    let lex_a = Token::lexer_from_file(&arena, f_a);
    let lex_b = Token::lexer_from_file(&arena, f_b);
    for x in lex_a.spanned() {
        println!("{x:?}");
    }
    println!();
    for x in lex_b.spanned() {
        println!("{x:?}");
    }
}
