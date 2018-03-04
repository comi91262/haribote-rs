use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "nask.pest"]
struct NaskParser;

pub fn parse(line: &str) -> Pairs<Rule> {
    NaskParser::parse(Rule::exp, line)
        .unwrap_or_else(|e| panic!("{}", e))
}

#[test]
fn db() {
    parses_to! {
        parser: NaskParser, 
        input: "DB 0x00, 0x00, 0x29",
        rule: Rule::db_exp,
        tokens: [
            db(0, 2),
            imm8(3, 7, [hex8(3, 7)]),
            imm8(9, 13, [hex8(9, 13)]),
            imm8(15, 19, [hex8(15, 19)])
        ]
    };
}

#[test]
fn resb() {
    parses_to! {
        parser: NaskParser, 
        input: "RESB 0x7dfe - $",
        rule: Rule::resb_exp,
        tokens: [
            resb(0, 4),
            imm16(5, 11, [hex16(5, 11)]),
            dollar(14, 15)
        ]
    };
}


#[test]
fn label() {
    parses_to! {
        parser: NaskParser, 
        input: "empty:  ",   
        rule: Rule::dist_label,
        tokens: [
            label(0, 5) 
        ]
    };
}

#[test]
fn jmp() {
    parses_to! {
        parser: NaskParser, 
        input: "JMP hoge",   
        rule: Rule::jmp_exp,
        tokens: [
            jmp(0, 3),
            label(4, 8)
        ]
    };
}


#[test]
fn je() {
    parses_to! {
        parser: NaskParser, 
        input: "JE hoge",   
        rule: Rule::je_exp,
        tokens: [
            je(0, 2),
            label(3, 7)
        ]
    };
}

#[test]
fn mov1() {
    parses_to! {
        parser: NaskParser, 
        input: "MOV SP,0x7c00",
        rule: Rule::mov_exp,
        tokens: [
            mov(0, 3),
            sp(4, 6),
            imm16(7, 13, [hex16(7, 13)])
        ]
    };
}
