# Little-Duck-2020
Scanner/Parser for a toy programming language called "LittleDuck2020" for my compiler class on Semester Jan-Jun 2022
Scanner/Parser was implemented with the grammar derived from the following picture:
![image](LittleDuck.png)

## LALRPOP
Parser was created using LALRPOP (https://lalrpop.github.io/lalrpop/index.html)
Parser has a deafult scanner that ignores whitespace characters.
Each grammar component is thoroughly testes in `tests/` folder, but you can run it and try it with your own inputs yourself!

### Run Parser

To run parser, you need to install Rust  
> https://www.rust-lang.org/tools/install

After installing Rust, open the project and run the command 

> cargo run

Parser will give a message if program was parsed correctly, or show the error found when parsing.
Parser takes file `input.txt` as input (File is found on `tests/` folder). 
Try changing this file to check out parser behaviour with different inputs.
