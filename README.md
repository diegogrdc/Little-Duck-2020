# Little-Duck-2020
Created a Scanner/Parser for a toy programming language called "LittleDuck2020" for my compiler class

## LALRPOP
Parser was created using LALRPOP (https://lalrpop.github.io/lalrpop/index.html)
Parser has a deafult scanner that ignores whitespace characters.
Scanner was impleemented with the defined grammar, as shown in the following picture:
![image](LittleDuck.png)
Each grammar component is thoroughly testes in `tests/` folder, but you can run it and try it with your own inputs yourself!

### Run Parser

To run parser, you need to install Rust  
> https://rust-lang.github.io/rustup/installation/index.html

After installing Rust, open the project and run the command 

> cargo run

Parser will give a message if program was parsed correctly, or show the error found when parsing.
Parser takes file `input.txt` as input (File is found on `tests/` folder). 
Try changing this file to check out parser behaviour with different inputs.
