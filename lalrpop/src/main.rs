use std::fs;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub littleduck); // synthesized by LALRPOP

fn main() {
    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res = littleduck::PROGRAMAParser::new().parse(&contents);
    match res {
        Ok(_) => print!("\n    Program was correctly parsed! It is written in LittleDuck2020!\n\n"),
        Err(error) => print!(
            "\nProblem parsing LittleDuck2020 file:\n\n    {:?}\n\n",
            error
        ),
    }
}
