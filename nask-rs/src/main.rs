use std::io::{BufWriter, Write};
use std::io::{Read, BufRead, BufReader, self};
use std::str::FromStr;

extern crate nask_rs;
use nask_rs::parser::parse;
use nask_rs::parser::Rule;

extern crate pest;

//#[derive(PartialEq, Debug)]
//enum Absyn<'a> {
//    DB(LinkedList<&'a str>),
//    RESB(u32)
//}
//

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    let mut rbuff = vec![];
    let mut current_address = 0;

    reader.read_to_end(&mut rbuff).unwrap();

    for line in rbuff.lines() {
        let line = line.unwrap();
        let mut pairs = parse(&line);

        let operator = pairs.next().unwrap();

        match operator.as_rule() {
            Rule::empty => {
            },
            Rule::org => {
                let operand = pairs.next().unwrap().into_span().as_str();
                current_address += u16::from_str_radix(operand, 16).unwrap();
            },
            Rule::db => {
                for pair in pairs {
                    let operand = pair.clone().into_span().as_str();
                    match pair.as_rule() {
                        Rule::hex2 => {
                            writer.write_fmt(format_args!(r"\x{}", operand)).unwrap();
                            current_address += 1;
                        },
                        Rule::num => {
                            let converted = u8::from_str(operand).unwrap();
                            writer.write_fmt(format_args!(r"\x{:02X}", converted)).unwrap();
                            current_address += 1;
                        },
                        Rule::str => {
                            for b in operand.bytes() {
                                writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap();
                                current_address += 1;
                            }
                        },
                        _ => unreachable!() 
                    }
                }
            },
            Rule::dw => {
                for pair in pairs {
                    let operand = pair.clone().into_span().as_str();
                    let b = {
                        match pair.as_rule() {
                            Rule::hex4 => {
                                u16::from_str_radix(operand, 16).unwrap()
                            },
                            Rule::num => {
                                u16::from_str_radix(operand, 10).unwrap()
                            },
                            _ => unreachable!()
                        }
                    };
                    let b0 =  b & 0x000F;
                    let b1 = (b & 0x00F0) >> 4;
                    let b2 = (b & 0x0F00) >> 8;
                    let b3 = (b & 0xF000) >> 12;
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
                    current_address += 2;
                }
            },
            Rule::dd => {
                for pair in pairs {
                    let operand = pair.clone().into_span().as_str();
                    let b = {
                        match pair.as_rule() {
                            Rule::hex8 => {
                                u32::from_str_radix(operand, 16).unwrap()
                            },
                            Rule::num => {
                                u32::from_str_radix(operand, 10).unwrap()
                            },
                            _ => unreachable!()
                        }
                    };
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
                    current_address += 4;
                }
            },
            Rule::resb => {
                let operand = pairs.next().unwrap();
                match operand.as_rule() {
                    Rule::hex4 => {
                        let b = u16::from_str_radix(operand.into_span().as_str(), 16).unwrap();
                        for _ in 0..(b - current_address) {
                            writer.write_fmt(format_args!(r"\x00")).unwrap();
                            current_address += 1;
                        }
                    },
                    Rule::num  => {
                        for _ in 0..u32::from_str(operand.into_span().as_str()).unwrap() {
                            writer.write_fmt(format_args!(r"\x00")).unwrap();
                            current_address += 1;
                        }

                    },
                    _ => unreachable!()
                }
            },
            Rule::int => {
                let operand = pairs.next().unwrap();
                match operand.as_rule() {
                    Rule::hex2 => {
                        let s = operand.clone().into_span().as_str();
                        writer.write_fmt(format_args!(r"\xCD\x{}", s)).unwrap();
                        current_address += 2;
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }


    }

}

