use id_arena::Arena;

use crate::{
    error::PrintingContext,
    // error::{MockContext, PrintingContext},
    fs::File,
    lexer::{tokens::Tokens, Token},
    parser::parse_item,
    transaction::ParserState,
};

mod ast;
mod error;
mod fs;
mod lexer;
mod parser;
mod span;
mod transaction;

type FileArena = Arena<File>;

fn main() {
    println!("Hello, world!");
    let mut arena: Arena<File> = Arena::new();
    let f_a = arena.alloc(File {
        name: "a".into(),
        contents: "fn main(): void {}".into(),
    });
    let f_b = arena.alloc(File {
        name: "b".into(),
        contents: "fn main(): bool {hello == b && a >= c; #a != 1 }".into(),
    });
    let lex_a = Token::lexer_from_file(&arena, f_a);
    let lex_b = Token::lexer_from_file(&arena, f_b);
    // for x in lex_a.spanned() {
    //     println!("{x:?}");
    // }
    // println!();
    // for x in lex_b.spanned() {
    //     println!("{x:?}");
    // }
    // println!();

    let parse_a = parse_item(&mut ParserState::new(
        PrintingContext::default(),
        Tokens::from(lex_a),
    ));
    println!("{parse_a:#?}");
    let parse_b = parse_item(&mut ParserState::new(
        PrintingContext::default(),
        Tokens::from(lex_b),
    ));
    println!("{parse_b:#?}")
}
