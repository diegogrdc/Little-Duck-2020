use std::fs;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("littleduck.l");
lrpar_mod!("littleduck.y");

fn main() {
    let lexerdef = littleduck_l::lexerdef();
    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lexer = lexerdef.lexer(&contents);
    let (res, errs) = littleduck_y::parse(&lexer);
    if errs.len() != 0 {
        println!("Lexing error found\n");
        for e in errs {
            println!("{}\n", e.pp(&lexer, &littleduck_y::token_epp));
        }
    } else {
        match res {
            Some(Ok(r)) => {
                println!(
                    "\n    Program was correctly parsed! It is written in LittleDuck2020!\n\n {}",
                    r
                )
            }
            _ => eprintln!("\nCode found an error!"),
        }
    }
}
