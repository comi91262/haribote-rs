extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "nask.pest"]
struct NaskParser;

use std::io::{BufWriter, Write};
use std::io::{Read, BufRead, BufReader, self};
use std::collections::LinkedList;
use std::str::FromStr;

//#[derive(PartialEq, Debug)]
//enum Absyn<'a> {
//    DB(LinkedList<&'a str>),
//    RESB(u32)
//}

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    let mut rbuff = vec![];

    reader.read_to_end(&mut rbuff).unwrap();

    for line in rbuff.lines() {
        let line = line.unwrap().clone();
        let mut pairs = 
            NaskParser::parse(Rule::exp, &line)
            .unwrap_or_else(|e| panic!("{}", e));

        let operator = pairs.next().unwrap();
        match operator.as_rule() {
            Rule::db => {
                let mut operands = LinkedList::<&str>::new();
                for pair in pairs {
                    operands.push_back(pair.clone().into_span().as_str());
                }
                for x in operands.iter() {
                    writer.write_fmt(format_args!(r"\x{}", x)).unwrap();
                }
            },
            Rule::resb => {
                let operand = pairs.next().unwrap();
                for _ in 0..u32::from_str(operand.into_span().as_str()).unwrap() {
                    writer.write_fmt(format_args!(r"\x00")).unwrap();
                }
            },
            _ => unreachable!()
        }

    }
}

