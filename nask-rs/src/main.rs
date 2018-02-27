use std::io::{BufWriter, Write};
use std::io::{Read, BufRead, BufReader, self};
use std::str::FromStr;
use std::collections::HashMap;

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
    let mut codes = vec![];
    let mut labels_map = HashMap::new();
    let mut address_map = HashMap::new();

    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    let mut rbuff = vec![];
    let mut current_address = 0;

    reader.read_to_end(&mut rbuff).unwrap();


    for line1 in rbuff.lines() {
        let line = line1.unwrap();
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
                            let converted = u8::from_str_radix(operand, 16).unwrap();
                            codes.push(converted);
                         //   writer.write_fmt(format_args!(r"\x{:02X}", converted)).unwrap();
                            current_address += 1;
                        },
                        Rule::num => {
                            let converted = u8::from_str(operand).unwrap();
                            codes.push(converted);
                          //  writer.write_fmt(format_args!(r"\x{:02X}", converted)).unwrap();
                            current_address += 1;
                        },
                        Rule::str => {
                            for b in operand.bytes() {
                                codes.push(b);
                          //     writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap();
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
                    let b1 = b & 0x00F0;//) >> 4;
                    let b2 = (b & 0x0F00) >> 8;
                    let b3 = (b & 0xF000) >> 8;
                    codes.push((b1 + b0) as u8);
                    codes.push((b3 + b2) as u8);
                    //writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    //writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
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
                    let b1 = (b & 0x000000F0);//) >>  4;
                    let b2 = (b & 0x00000F00) >>  8;
                    let b3 = (b & 0x0000F000) >>  8;
                    let b4 = (b & 0x000F0000) >> 16;
                    let b5 = (b & 0x00F00000) >> 16;
                    let b6 = (b & 0x0F000000) >> 24;
                    let b7 = (b & 0xF0000000) >> 24;
                    codes.push((b1 + b0) as u8);
                    codes.push((b3 + b2) as u8);
                    codes.push((b5 + b4) as u8);
                    codes.push((b7 + b6) as u8);
                    //writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b1, b0)).unwrap();
                    //writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b3, b2)).unwrap();
                    //writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b5, b4)).unwrap();
                    //writer.write_fmt(format_args!(r"\x{:01X}{:01X}", b7, b6)).unwrap();
                    current_address += 4;
                }
            },
            Rule::resb => {
                let operand = pairs.next().unwrap();
                match operand.as_rule() {
                    Rule::hex4 => {
                        let b = u16::from_str_radix(operand.into_span().as_str(), 16).unwrap();
                        for _ in 0..(b - current_address) {
                            codes.push(0);
                    //        writer.write_fmt(format_args!(r"\x00")).unwrap();
                            current_address += 1;
                        }
                    },
                    Rule::num  => {
                        for _ in 0..u32::from_str(operand.into_span().as_str()).unwrap() {
                            codes.push(0);
                     //       writer.write_fmt(format_args!(r"\x00")).unwrap();
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
                        let converted = u8::from_str_radix(s, 16).unwrap();
                      //  writer.write_fmt(format_args!(r"\xCD\x{}", s)).unwrap();
                        codes.push(205);
                        codes.push(converted);
                        current_address += 2;
                    },
                    _ => unreachable!()
                }
            },
            Rule::jmp => {
                let label = pairs.next().unwrap().clone().into_span().as_str();
                codes.push(235);
                codes.push(255);  //tmp
                address_map.insert((current_address+1) as u32, label.to_string());
                current_address += 2;
            },
           // Rule::je => {
           //     let label = pairs.next().unwrap().clone().into_span().as_str();
           //     labels_map.insert(label, 0);
           // },
            Rule::label => {
                let s = operator.clone().into_span().as_str();
                labels_map.insert(s.to_string(), current_address);
            },
            Rule::hlt => {
                codes.push(244); //F4
                current_address += 1;
            },
            _ => unreachable!()
        }
    }
    //check: throw exception if there is a label to which no address is assigned.
    for (label, &address) in labels_map.iter() {
        if address == 0 {
            panic!("Some label has no value.");
        }
    }

    for (idx, &b) in codes.iter().enumerate() {
        if b == 255 {
            match address_map.get(&((idx  as u32) + 31744)) {
                Some(label) => {
                    let address = labels_map.get(label).unwrap();
                    let bb = 0x00FF & address;
                    writer.write_fmt(format_args!(r"\x{:02X}", &(bb - 2))).unwrap(); //-2 ??
                },
                None => {
                    writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap();

                }
            }
        } else {
            writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap();
        }
    }


}

