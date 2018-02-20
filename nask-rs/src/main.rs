use std::io::{BufWriter, Write};
use std::io::{Read, BufRead, BufReader, self};
use std::collections::LinkedList;
use std::str::FromStr;

extern crate nask_rs;
use nask_rs::parser::parse;
use nask_rs::parser::Rule;

//#[derive(PartialEq, Debug)]
//enum Absyn<'a> {
//    DB(LinkedList<&'a str>),
//    RESB(u32)
//}

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    let mut rbuff = vec![];
    let mut line_count = 0;

    reader.read_to_end(&mut rbuff).unwrap();

    for line in rbuff.lines() {
        let line = line.unwrap();
        let mut pairs = parse(&line);

        let operator = pairs.next().unwrap();

        match operator.as_rule() {
            Rule::org => {
                 
            },
            Rule::db => {
                let mut operands1 = LinkedList::<&str>::new();
                let mut operands2 = LinkedList::<&str>::new();
                let mut operands3 = LinkedList::<&str>::new();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::hex2 => {
                            operands1.push_back(pair.clone().into_span().as_str());
                        },
                        Rule::str => {
                            operands2.push_back(pair.clone().into_span().as_str());
                        },
                        Rule::num => {
                            operands3.push_back(pair.clone().into_span().as_str());

                        },
                        _ => unreachable!()
                    }
                }
                for x in operands1.iter() {
                    writer.write_fmt(format_args!(r"\x{}", x)).unwrap();
                    line_count += 1;

                }
                //str
                for x in operands2.iter() {
                    for s in x.bytes(){
                        writer.write_fmt(format_args!(r"\x{:02X}", s)).unwrap();
                        line_count += 1;
                    }
                }
                //num 
                for x in operands3.iter() {
                    let a = u8::from_str(x).unwrap();
                    writer.write_fmt(format_args!(r"\x{:02X}", a)).unwrap();
                    line_count += 1;
                }

            },
            Rule::dw => {
                let mut operands1 = LinkedList::<&str>::new();
                let mut operands3 = LinkedList::<&str>::new();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::hex4 => {
                            operands3.push_back(pair.clone().into_span().as_str());
                        },
                        Rule::num => {
                            operands1.push_back(pair.clone().into_span().as_str());

                        },
                        _ => unreachable!()
                    }
                }
                //num
                for x in operands1.iter() {
                    let b = u16::from_str_radix(x, 10).unwrap();
                    let b0 =  b & 0x000F;
                    let b1 = (b & 0x00F0) >> 4;
                    let b2 = (b & 0x0F00) >> 8;
                    let b3 = (b & 0xF000) >> 12;
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
                    line_count += 2;
                }
                //hex4 0xaaaa
                for x in operands3.iter() {
                    let b = u16::from_str_radix(x, 16).unwrap();
                    let b0 =  b & 0x000F;
                    let b1 = (b & 0x00F0) >> 4;
                    let b2 = (b & 0x0F00) >> 8;
                    let b3 = (b & 0xF000) >> 12;
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
                    line_count += 2;
                }

            },
            Rule::dd => {
                let mut operands1 = LinkedList::<&str>::new();
                let mut operands3 = LinkedList::<&str>::new();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::hex8 => {
                            operands3.push_back(pair.clone().into_span().as_str());
                        },
                        Rule::num => {
                            operands1.push_back(pair.clone().into_span().as_str());

                        },
                        _ => unreachable!()
                    }
                }
                //num 
                for x in operands1.iter() {
                    let b = u32::from_str_radix(x, 10).unwrap();

                    let b0 =  b & 0x0000000F;
                    let b1 = (b & 0x000000F0) >>  4;
                    let b2 = (b & 0x00000F00) >>  8;
                    let b3 = (b & 0x0000F000) >> 12;
                    let b4 = (b & 0x000F0000) >> 16;
                    let b5 = (b & 0x00F00000) >> 20;
                    let b6 = (b & 0x0F000000) >> 24;
                    let b7 = (b & 0xF0000000) >> 28;
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b5, b4)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b7, b6)).unwrap();
                    line_count += 4;
                }

                for x in operands3.iter() {

                    let b = u32::from_str_radix(x, 16).unwrap();

                    let b0 =  b & 0x0000000F;
                    let b1 = (b & 0x000000F0) >>  4;
                    let b2 = (b & 0x00000F00) >>  8;
                    let b3 = (b & 0x0000F000) >> 12;
                    let b4 = (b & 0x000F0000) >> 16;
                    let b5 = (b & 0x00F00000) >> 20;
                    let b6 = (b & 0x0F000000) >> 24;
                    let b7 = (b & 0xF0000000) >> 28;
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b5, b4)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b7, b6)).unwrap();
                    line_count += 4;

                }

            },
            Rule::resb => {
                let operand = pairs.next().unwrap();
                match operand.as_rule() {
                    Rule::hex4 => {
                        let b = u16::from_str_radix(operand.into_span().as_str(), 16).unwrap();
                        for _ in 0..(b - line_count) {
                            writer.write_fmt(format_args!(r"\x00")).unwrap();
                            line_count += 1;
                        }

                    },
                    Rule::num  => {
                        for _ in 0..u32::from_str(operand.into_span().as_str()).unwrap() {
                            writer.write_fmt(format_args!(r"\x00")).unwrap();
                        line_count += 1;
                        }

                    },
                    _ => unreachable!()
                }
            },
            _ => println!("unreachable")
        }


    }

}

