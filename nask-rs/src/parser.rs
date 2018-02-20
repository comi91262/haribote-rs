use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "nask.pest"]
struct NaskParser;

pub fn parse(line: &str) -> Pairs<Rule> {
    NaskParser::parse(Rule::exp, line)
        .unwrap_or_else(|e| panic!("{}", e))
}


