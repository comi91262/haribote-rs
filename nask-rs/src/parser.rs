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

