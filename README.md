# Little-Duck-2020
Lexers and Scanners/Parsers for a toy programming language called "LittleDuck2020" for my compiler class on Semester Jan-Jun 2022
Lexers and Scanners/Parsers  were implemented with the grammar derived from the language defined from the following picture:
![image](LittleDuck.png)

## Tools used for code analysis 

### LALRPOP
First tool used to create a parser was LALRPOP (https://lalrpop.github.io/lalrpop/index.html)
Parser has a deafult scanner that ignores whitespace characters.
Each grammar component is thoroughly testes in `tests/` folder, but you can run it and try it with your own inputs yourself!

### grmtools

Second tool used to create a lexer and parser was grmtools (https://softdevteam.github.io/grmtools/master/book/index.html). 
This tool uses an advanced implementation of Lex&Yacc to analyze a grammar through a lexer and a parser. You can try it yourself by changing the input.txt file found on `src` folder. 

## Running lexers/parsers 

To run parsers, you need to install Rust  
> https://www.rust-lang.org/tools/install

After installing Rust, open any folder (LALRPOP, grmtools), depending on which tool you would like to use for parsing, and run the command 

> cargo run

Parser will give a message if program was parsed correctly, or show the error found when parsing.
Parser takes file `input.txt` as input (File is found on `src/` folder). 
Try changing this file to check out parser behavior with different inputs.