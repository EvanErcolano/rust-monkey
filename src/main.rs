// cargo and rust don't pull in all your rust files
// it just pulls in main lib or mod files
// you tell rust and cargo what to pull in with mod statements

// will check for lexer.rs or check whether theres a dir called lexer
mod lexer;
mod token;
mod parser;
mod ast;
// top level declarations  telling rust plz compile this thing
// can refer to something .rs or a sub folder with a mod.rs inside of it
// mod declarares the file exists

// use statements
// there is something else in our file tree in the source dir
// and we want to use it

use lexer::Lexer;

fn main() {
    println!("Lexer starting up....");
    let lexer = Lexer::new(&String::from("5 + 5")).unwrap();
    for token in lexer {
        println!("{:?}", token);
    }
    // let p = parser::Parser::new(lexer);
    // println!("{:?}", p);
}
