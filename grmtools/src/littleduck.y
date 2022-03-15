%start Programa 
%%

Programa -> Result<String, ()>:
    'PROGRAM' 'ID' 'SEMICOLON' Programa2 { Ok("".to_string()) }
    ;

Programa2 -> Result<String, ()>:
    Bloque { Ok("".to_string()) } 
    | Vars Bloque { Ok("".to_string()) }
    ;

Vars -> Result<String, ()>:
    'VAR' Vars2 { Ok("".to_string()) }
    ;

Vars2 -> Result<String, ()>:
    'ID' 'COMMA' Vars2 { Ok("".to_string()) }
    | 'ID' 'COLON' Tipo 'SEMICOLON' Vars2 { Ok("".to_string()) }
    | 'ID' 'COLON' Tipo 'SEMICOLON' { Ok("".to_string()) }
    ;

Bloque -> Result<String, ()>:
    'LEFTBRACKET' Estatutos 'RIGTHBRACKET' { Ok("".to_string()) }
    | 'LEFTBRACKET' 'RIGTHBRACKET' { Ok("".to_string()) }
    ;

Estatutos -> Result<String, ()>:
    Estatuto { Ok("".to_string()) }
    | Estatutos Estatuto { Ok("".to_string()) }
    ;

Estatuto -> Result<String, ()>:
    Asignacion { Ok("".to_string()) }
    | Condicion { Ok("".to_string()) }
    | Escritura { Ok("".to_string()) }
    ;

Condicion -> Result<String, ()>:
    'IF' 'LEFTPARENTHESES' Expresion 'RIGHTPARENTHESES' 
    Bloque 'SEMICOLON' { Ok("".to_string()) }
    | 'IF' 'LEFTPARENTHESES' Expresion 'RIGHTPARENTHESES' 
    Bloque 'ELSE' Bloque 'SEMICOLON' { Ok("".to_string()) }
    ;

Asignacion -> Result<String, ()>:
    'ID' 'EQUALSIGN' Expresion 'SEMICOLON' { Ok("".to_string()) }
    ;

Escritura -> Result<String, ()>:
    'PRINT' 'LEFTPARENTHESES' Escritura2 'RIGHTPARENTHESES' 'SEMICOLON' { Ok("".to_string()) }
    ;

Escritura2 -> Result<String, ()>:
    Expresion { Ok("".to_string()) }
    | Expresion 'COMMA' Escritura2 { Ok("".to_string()) }
    | 'CTESTR' { Ok("".to_string()) }
    | 'CTESTR' 'COMMA' Escritura2 { Ok("".to_string()) }
    ;

Tipo -> Result<String, ()>:
    'INT' { Ok("".to_string()) }
    | 'FLOAT' { Ok("".to_string()) }
    ;

Expresion -> Result<String, ()>:
    Exp { Ok("".to_string()) }
    | Exp Cmp Exp { Ok("".to_string()) }
    ;

Exp -> Result<String, ()>:
    Termino { Ok("".to_string()) }
    | Termino 'PLUSSIGN' Exp { Ok("".to_string()) }
    | Termino 'MINUSSIGN' Exp { Ok("".to_string()) }
    ;

Cmp -> Result<String, ()>:
    'GREATERSIGN' { Ok("".to_string()) }
    | 'LESSSIGN' { Ok("".to_string()) }
    | 'NOTEQUALSIGN' { Ok("".to_string()) }
    ;

Termino -> Result<String, ()>:
    Factor { Ok("".to_string()) }
    | Factor 'MULTIPLICATIONSIGN' Termino { Ok("".to_string()) }
    | Factor 'DIVISIONSIGN' Termino { Ok("".to_string()) }
    ;

Factor -> Result<String, ()>:
    'LEFTPARENTHESES' Exp 'RIGHTPARENTHESES' { Ok("".to_string()) }
    | Factor2 { Ok("".to_string()) }
    ;

Factor2 -> Result<String, ()>:
    Varcte { Ok("".to_string()) }
    | 'PLUSSIGN' Varcte { Ok("".to_string()) }
    | 'MINUSSIGN' Varcte { Ok("".to_string()) }
    ;

Varcte -> Result<String, ()>:
    'ID' { Ok("".to_string()) }
    | 'CTEI' { Ok("".to_string()) }
    | 'CTEF' { Ok("".to_string()) }
    ;

%%