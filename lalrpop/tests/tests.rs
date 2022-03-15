#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub littleduck); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_programa_parsing() {
        // Correct PROGRAMAs
        assert!(littleduck::PROGRAMAParser::new()
            .parse(r#"program HELLO_WORLD; {print("Hello World");}"#)
            .is_ok());
        assert!(littleduck::PROGRAMAParser::new()
            .parse("program PROGRAM2; var id: int; id2, id3: float; {print(id, id2 + id3);}")
            .is_ok());
        // Incorrect PROGRAMAs
        assert!(littleduck::PROGRAMAParser::new()
            .parse("var id: int; id2, id3: float; {print(id, id2 + id3);}")
            .is_err());
        assert!(littleduck::PROGRAMAParser::new()
            .parse("program PROGRAM2; {print(id, id2 + id3);} var id: int;")
            .is_err());
    }

    #[test]
    fn test_vars_parsing() {
        // Correct VARSs
        assert!(littleduck::VARSParser::new()
            .parse("var id1 : int;")
            .is_ok());
        assert!(littleduck::VARSParser::new()
            .parse("var id1, id2, id3 : float;")
            .is_ok());
        assert!(littleduck::VARSParser::new()
            .parse("var id1, id2, id3 : float; id4, id5 : int; id6 : float;")
            .is_ok());
        // Incorrect VARSs
        assert!(littleduck::VARSParser::new().parse("id1 : int;").is_err());
        assert!(littleduck::VARSParser::new()
            .parse("var id1 : int; var id2: float;")
            .is_err());
    }
    #[test]
    fn test_vars2_parsing() {
        // Correct VARS2s
        assert!(littleduck::VARS2Parser::new().parse("id1 : int;").is_ok());
        assert!(littleduck::VARS2Parser::new()
            .parse("id1, id2, id3 : float;")
            .is_ok());
        assert!(littleduck::VARS2Parser::new()
            .parse("id1, id2, id3 : float; id4, id5 : int; id6 : float;")
            .is_ok());
        // Incorrect VARS2s
        assert!(littleduck::VARS2Parser::new().parse("id1, id2;").is_err());
        assert!(littleduck::VARS2Parser::new().parse("id1 : int").is_err());
        assert!(littleduck::VARS2Parser::new().parse(": int;").is_err());
    }

    #[test]
    fn test_bloque_parsing() {
        // Correct BLOQUEs
        assert!(littleduck::BLOQUEParser::new().parse("{}").is_ok());
        assert!(littleduck::BLOQUEParser::new()
            .parse("{id = (12 * 23); id2 = 12.54;}")
            .is_ok());
        assert!(littleduck::BLOQUEParser::new()
            .parse("{print(1, 2, 3); print((12 + 15 * 3 - 2));}")
            .is_ok());
        // Incorrect BLOQUEs
        assert!(littleduck::BLOQUEParser::new()
            .parse("id = (12 * 23); id2 = 12.54;")
            .is_err());
        assert!(littleduck::BLOQUEParser::new()
            .parse("if(12){}else{};")
            .is_err());
        assert!(littleduck::BLOQUEParser::new().parse("{").is_err());
    }

    #[test]
    fn test_estatutos_parsing() {
        // Correct ESTATUTOS
        assert!(littleduck::ESTATUTOSParser::new()
            .parse("id = (12 * 23); id2 = 12.54;")
            .is_ok());
        assert!(littleduck::ESTATUTOSParser::new()
            .parse("if(12 > 5){id = 1;} else {id = 2;}; print(id);")
            .is_ok());
        assert!(littleduck::ESTATUTOSParser::new()
            .parse(r#"print("hola");"#)
            .is_ok());
        // Incorrect ESTATUTOS
        assert!(littleduck::ESTATUTOSParser::new().parse("").is_err());
        assert!(littleduck::ESTATUTOSParser::new()
            .parse("(12 * 5);")
            .is_err());
        assert!(littleduck::ESTATUTOSParser::new()
            .parse("id = 12; id2 = 25")
            .is_err());
    }

    #[test]
    fn test_estatuto_parsing() {
        // Correct ESTATUTO
        assert!(littleduck::ESTATUTOParser::new()
            .parse("id = (12 * 23);")
            .is_ok());
        assert!(littleduck::ESTATUTOParser::new()
            .parse("print(12);")
            .is_ok());
        assert!(littleduck::ESTATUTOParser::new()
            .parse("if(12 > 0){};")
            .is_ok());
        assert!(littleduck::ESTATUTOParser::new()
            .parse("if(12){}else{};")
            .is_ok());
        // INCORRECT ESTATUTO
        assert!(littleduck::ESTATUTOParser::new()
            .parse("(12 > 15);")
            .is_err());
        assert!(littleduck::ESTATUTOParser::new()
            .parse("id = 1; id2 = 3;")
            .is_err());
        assert!(littleduck::ESTATUTOParser::new()
            .parse("4 * 5 + 2 / 3;")
            .is_err());
    }

    #[test]
    fn test_asignacion_parsing() {
        // Correct ASIGNACIONs
        assert!(littleduck::ASIGNACIONParser::new()
            .parse("id = (12 * 23);")
            .is_ok());
        assert!(littleduck::ASIGNACIONParser::new()
            .parse("id = 25.4;")
            .is_ok());
        assert!(littleduck::ASIGNACIONParser::new()
            .parse("idOk = 4 > 3;")
            .is_ok());
        // Incorrect ASIGNACIONs
        assert!(littleduck::ASIGNACIONParser::new()
            .parse("id = (12 * 23)")
            .is_err());
        assert!(littleduck::ASIGNACIONParser::new()
            .parse("id == id2;")
            .is_err());
        assert!(littleduck::ASIGNACIONParser::new()
            .parse("_id = id2;")
            .is_err());
    }
    #[test]
    fn test_escritura_parsing() {
        // Correct ESCRITURAs
        assert!(littleduck::ESCRITURAParser::new()
            .parse("print(12);")
            .is_ok());
        assert!(littleduck::ESCRITURAParser::new()
            .parse("print(12, 15 * 4, (id * 23 + 14));")
            .is_ok());
        assert!(littleduck::ESCRITURAParser::new()
            .parse(r#"print("Hola");"#)
            .is_ok());
        assert!(littleduck::ESCRITURAParser::new()
            .parse(r#"print("Hola", 12 * 5, "otro text", -id4);"#)
            .is_ok());
        // Incorrect ESCRITURAs
        assert!(littleduck::ESCRITURAParser::new()
            .parse("print(12)")
            .is_err());
        assert!(littleduck::ESCRITURAParser::new()
            .parse("print 12 * 15")
            .is_err());
        assert!(littleduck::ESCRITURAParser::new()
            .parse("print();")
            .is_err());
        assert!(littleduck::ESCRITURAParser::new()
            .parse("print(12 * 5,);")
            .is_err());
    }

    #[test]
    fn test_tipo_parsing() {
        // Correct TIPOs
        assert!(littleduck::TIPOParser::new().parse("int").is_ok());
        assert!(littleduck::TIPOParser::new().parse("float").is_ok());
        // Incorrect TIPOs
        assert!(littleduck::TIPOParser::new().parse("INT").is_err());
        assert!(littleduck::TIPOParser::new().parse("FLOAT").is_err());
    }

    #[test]
    fn test_expresion_parsing() {
        // Correct EXPRESIONs
        assert!(littleduck::EXPRESIONParser::new()
            .parse("(12 + 23) > id")
            .is_ok());
        assert!(littleduck::EXPRESIONParser::new()
            .parse("- id * + 13.56 < id")
            .is_ok());
        assert!(littleduck::EXPRESIONParser::new()
            .parse("- id * + 13.56 <> ((12) * -id)")
            .is_ok());
        assert!(littleduck::EXPRESIONParser::new()
            .parse("+-23 > -id")
            .is_ok());
        // Incorrect EXPRESIONs
        assert!(littleduck::EXPRESIONParser::new()
            .parse("+-id > 12")
            .is_err());
        assert!(littleduck::EXPRESIONParser::new()
            .parse("---12.4 > 35")
            .is_err());
        assert!(littleduck::EXPRESIONParser::new()
            .parse("> variable")
            .is_err());
        assert!(littleduck::EXPRESIONParser::new().parse("<>").is_err());
    }

    #[test]
    fn test_exp_parsing() {
        // Correct EXPs
        assert!(littleduck::EXPParser::new()
            .parse("(12 + 13) * 25 - id1 * (15 - 12)")
            .is_ok());
        assert!(littleduck::EXPParser::new()
            .parse("1.4 * id / 13 + 23 * 45 - 100 * 0.97 + (- id)")
            .is_ok());
        assert!(littleduck::EXPParser::new()
            .parse("((variable) * ((34) + 12))")
            .is_ok());
        // Incorrect EXPs
        assert!(littleduck::EXPParser::new().parse("()").is_err());
        assert!(littleduck::EXPParser::new().parse("5 ? 12").is_err());
        assert!(littleduck::EXPParser::new()
            .parse("((12)(25)) * 23")
            .is_err());
    }

    #[test]
    fn test_termino_parsing() {
        // Correct TERMINOs
        assert!(littleduck::TERMINOParser::new().parse("id").is_ok());
        assert!(littleduck::TERMINOParser::new().parse("id * 12").is_ok());
        assert!(littleduck::TERMINOParser::new()
            .parse("(12.5 * id23) / 3")
            .is_ok());
        assert!(littleduck::TERMINOParser::new()
            .parse("(2 * 2 / 2) * 15 / (33 * 12)")
            .is_ok());
        // Incorrect TERMINOs
        assert!(littleduck::TERMINOParser::new().parse("").is_err());
        assert!(littleduck::TERMINOParser::new().parse("()").is_err());
        assert!(littleduck::TERMINOParser::new()
            .parse("(12 * 15) * ")
            .is_err());
        assert!(littleduck::TERMINOParser::new()
            .parse("(12 * 15) / ()")
            .is_err());
        assert!(littleduck::TERMINOParser::new().parse("* 27").is_err());
        assert!(littleduck::TERMINOParser::new().parse("/ id").is_err());
    }

    #[test]
    fn test_factor_parsing() {
        // Correct EXPs
        assert!(littleduck::FACTORParser::new().parse("idAlone").is_ok());
        assert!(littleduck::FACTORParser::new().parse("+2.31").is_ok());
        assert!(littleduck::FACTORParser::new().parse("15").is_ok());
        assert!(littleduck::FACTORParser::new().parse("(15)").is_ok());
        assert!(littleduck::FACTORParser::new().parse("(-32.5)").is_ok());
        assert!(littleduck::FACTORParser::new().parse("(-id)").is_ok());
        // Inorrect EXPs
        assert!(littleduck::FACTORParser::new().parse("()").is_err());
        assert!(littleduck::FACTORParser::new().parse("").is_err());
        assert!(littleduck::FACTORParser::new().parse(")(").is_err());
        assert!(littleduck::FACTORParser::new().parse("(id").is_err());
        assert!(littleduck::FACTORParser::new().parse("22)").is_err());
    }

    #[test]
    fn test_factor2_parsing() {
        // Correct FACTOR2s
        assert!(littleduck::FACTOR2Parser::new().parse("idAlone").is_ok());
        assert!(littleduck::FACTOR2Parser::new().parse("+idAlone").is_ok());
        assert!(littleduck::FACTOR2Parser::new().parse("-123").is_ok());
        assert!(littleduck::FACTOR2Parser::new().parse("+2.31").is_ok());
        assert!(littleduck::FACTOR2Parser::new().parse("15").is_ok());
        // Incorrect FACTOR2s
        assert!(littleduck::FACTOR2Parser::new().parse("id +").is_err());
        assert!(littleduck::FACTOR2Parser::new().parse("123 +").is_err());
        assert!(littleduck::FACTOR2Parser::new().parse("1.2 -").is_err());
        assert!(littleduck::FACTOR2Parser::new().parse("+ 1id").is_err());
    }

    #[test]
    fn test_cmp_parsing() {
        // Correct CMPs
        assert!(littleduck::CMPParser::new().parse("<").is_ok());
        assert!(littleduck::CMPParser::new().parse(">").is_ok());
        assert!(littleduck::CMPParser::new().parse("<>").is_ok());
        // Incorrect CMPs
        assert!(littleduck::CMPParser::new().parse("><").is_err());
        assert!(littleduck::CMPParser::new().parse(">>").is_err());
        assert!(littleduck::CMPParser::new().parse("<<").is_err());
        assert!(littleduck::CMPParser::new().parse("<=").is_err());
        assert!(littleduck::CMPParser::new().parse(">=").is_err());
    }

    #[test]
    fn test_varcte_parsing() {
        // Correct VARCTEs
        assert!(littleduck::VARCTEParser::new().parse("validID123").is_ok());
        assert!(littleduck::VARCTEParser::new().parse("+115").is_ok());
        assert!(littleduck::VARCTEParser::new().parse("23.45").is_ok());
        // Incorrect VARCTEs
        assert!(littleduck::VARCTEParser::new()
            .parse(r#""Do not allow strings""#)
            .is_err());
        assert!(littleduck::VARCTEParser::new().parse("1InvalidID").is_err());
        assert!(littleduck::VARCTEParser::new().parse("++12").is_err());
        assert!(littleduck::VARCTEParser::new().parse("1.2.3").is_err());
    }

    #[test]
    fn test_id_parsing() {
        // Correct IDs
        assert!(littleduck::IDParser::new()
            .parse("this_Is_A_Valid_ID_123")
            .is_ok());
        assert!(littleduck::IDParser::new().parse("id").is_ok());
        assert!(littleduck::IDParser::new().parse("variable").is_ok());
        // Incorrect IDs
        assert!(littleduck::IDParser::new().parse("123Invalid_ID").is_err());
        assert!(littleduck::IDParser::new().parse("_Invalid2").is_err());
    }

    #[test]
    fn test_ctei_parsing() {
        // Correct CTEIs
        assert!(littleduck::CTEIParser::new().parse("123").is_ok());
        assert!(littleduck::CTEIParser::new().parse("+123").is_ok());
        assert!(littleduck::CTEIParser::new().parse("-0").is_ok());
        assert!(littleduck::CTEIParser::new().parse("999999").is_ok());
        // Incorrect CTEIs
        assert!(littleduck::CTEIParser::new().parse("+-1").is_err());
        assert!(littleduck::CTEIParser::new().parse("1.23").is_err());
        assert!(littleduck::CTEIParser::new().parse("words").is_err());
    }

    #[test]
    fn test_ctef_parsing() {
        // Correct CTEFs
        assert!(littleduck::CTEFParser::new().parse("123.1").is_ok());
        assert!(littleduck::CTEFParser::new().parse("0.15").is_ok());
        assert!(littleduck::CTEFParser::new().parse("3.").is_ok());
        assert!(littleduck::CTEFParser::new().parse("-15.").is_ok());
        assert!(littleduck::CTEFParser::new().parse("+101.101").is_ok());
        // Incorrect CTEFs
        assert!(littleduck::CTEFParser::new().parse("3").is_err());
        assert!(littleduck::CTEFParser::new().parse("4.4.4").is_err());
        assert!(littleduck::CTEFParser::new().parse("words").is_err());
    }
    #[test]
    fn test_ctestr_parsing() {
        // Correct CTESTRs
        assert!(littleduck::CTESTRParser::new()
            .parse(r#""I am a string""#)
            .is_ok());
        assert!(littleduck::CTESTRParser::new().parse(r#""WOW!""#).is_ok());
        assert!(littleduck::CTESTRParser::new()
            .parse(r#""123._ <'123 lots of weird chars""#)
            .is_ok());
        // Incorrect CTESTRs
        assert!(littleduck::CTESTRParser::new()
            .parse("Where are my quotes?")
            .is_err());
        assert!(littleduck::CTESTRParser::new()
            .parse(r#"Cant have quotes like this " in the middle"#)
            .is_err());
    }
}
