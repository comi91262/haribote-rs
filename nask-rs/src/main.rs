use std::io::{BufWriter, Write};
use std::io::{Read, BufRead, BufReader, self};
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
//

// TODO to add a limit to num
fn to_u8(imm: &str, rule: Rule) -> io::Result<u8> {
    match rule {
        Rule::hex8 => {
            u8::from_str_radix(imm, 16).unwrap()
        },
        Rule::dec8 => {
            u8::from_str_radix(imm, 10).unwrap()
        },
        _ => unreachable!() 
    }
}

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
                let op = pairs.next().unwrap().as_str();
                //current_address += u16::from_str_radix(op, 16).unwrap();
            },
            Rule::db => {
                for op in pairs.next() {
                    match op.as_rule() {
                        Rule::imm8 => {
                            op.into_inner().next().unwrap();
                            match op.into_inner().next() {
                                Some(inner) => {
                                    let a = inner.as_str();
                                    let b = inner.as_rule();
                                    codes.push(to_u8(a, b));
                                },
                                _ => unreachable!() 
                            }
                            current_address += 1;
                        },
                        Rule::str => {
                            for b in op.as_str().bytes() {
                                codes.push(b);
                                current_address += 1;
                            }
                        },
                        _ => unreachable!() 
                    }
                }
            },
            Rule::dw => {
                let op1 = pairs.next().unwrap();
                let mut b = 0x0000;
                match op1.as_rule() {
                    Rule::imm16 => {
                        for inner in op1.into_inner() {
                            match inner.as_rule() {
                                Rule::hex16 => {
                                    b = u16::from_str_radix(inner.as_str(), 16).unwrap()
                                },
                                Rule::dec16 => {
                                    b = u16::from_str_radix(inner.as_str(), 10).unwrap()
                                },
                                _ => unreachable!() 
                            }
                        }
                    }
                    _ => unreachable!()
                }
                let b10 =  b & 0x00FF;
                let b32 = (b & 0xFF00) >> 8;
                codes.push(b10 as u8);
                codes.push(b32 as u8);
                current_address += 2;
            },
            Rule::dd => {
                let op1 = pairs.next().unwrap();
                let mut b = 0x0000;
                match op1.as_rule() {
                    Rule::imm32 => {
                        for inner in op1.into_inner() {
                            match inner.as_rule() {
                                Rule::hex32 => {
                                    b = u32::from_str_radix(inner.as_str(), 16).unwrap()
                                },
                                Rule::dec32 => {
                                    b = u32::from_str_radix(inner.as_str(), 10).unwrap()
                                },
                                _ => unreachable!() 
                            }
                        }
                    }
                    _ => unreachable!()
                }

                let b10 =  b & 0x000000FF;
                let b32 = (b & 0x0000FF00) >>  8;
                let b54 = (b & 0x00FF0000) >> 16;
                let b76 = (b & 0xFF000000) >> 24;
                codes.push(b10 as u8);
                codes.push(b32 as u8);
                codes.push(b54 as u8);
                codes.push(b76 as u8);
                current_address += 4;
            },
            Rule::resb => {
                let op = pairs.next().unwrap();
                match op.as_rule() {
                    Rule::imm16 => {
                        for inner in op.into_inner() {
                            match inner.as_rule() {
                                Rule::hex16 => {
                                    let b = u16::from_str_radix(inner.as_str(), 16).unwrap();
                                    for _ in 0..(b - current_address) {
                                        codes.push(0);
                                        current_address += 1;
                                    }
                                },
                                Rule::dec16 => {
                                    let n = u16::from_str_radix(inner.as_str(), 10).unwrap();
                                    for _ in 0..n {
                                        codes.push(0);
                                        current_address += 1;
                                    }
                                },
                                _ => unreachable!() 
                            }
                        }
                    }
                    _ => unreachable!()
                }
            },
            Rule::int => {
               // let operand = pairs.next().unwrap();
               // match operand.as_rule() {
                  //  Rule::hex2 => {
                  //      let s = operand.clone().into_span().as_str();
                  //      let converted = u8::from_str_radix(s, 16).unwrap();
                  //    //  writer.write_fmt(format_args!(r"\xCD\x{}", s)).unwrap();
                  //      codes.push(205);
                  //      codes.push(converted);
                  //      current_address += 2;
                  //  },
            //    _ => unreachable!()
               // }
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
            Rule::cmp => {
//                let op1 = pairs.next().unwrap();
//                let op2 = pairs.next().unwrap();
//
//                match pairs.next().unwrap().as_rule() {
//                    Rule::al => {
//                        codes.push(0x3C); 
//                        let operand = pairs.next().unwrap().clone().into_span().as_str();
//                        let converted = u8::from_str_radix(operand, 10).unwrap();
//                        codes.push(converted);
//                        current_address += 2;
//                    },
//                    _ => unreachable!()
//                }
            },
            Rule::add => {
//                let op1 = pairs.next().unwrap();
//                let op2 = pairs.next().unwrap();
//
//                match op1.as_rule() {
//                    Rule::si => {
//                            codes.push(0x83); 
//                            codes.push(0xC6);
//                            let converted = u8::from_str_radix(op2.as_str(), 10).unwrap();
//                            codes.push(converted);
//                            current_address += 3;
//                    },
//                    _ => unreachable!()
//                }
            },
            Rule::mov => {
                let op1 = pairs.next().unwrap();
                let op2 = pairs.next().unwrap();
                match op1.as_rule() {
                    Rule::ss => {
                        match op2.as_rule() {
                            Rule::ax => {
                                codes.push(0x8E); 
                                codes.push(0xD0);
                                current_address += 2;

                            },
                            _ => unreachable!()
                        }
                    },
                    Rule::ds => {
                        match op2.as_rule() {
                            Rule::ax => {
                                codes.push(0x8E); 
                                codes.push(0xD8);
                                current_address += 2;
                            },
                            _ => unreachable!()
                        }
                    },
                    Rule::es => {
                        match op2.as_rule() {
                            Rule::ax => {
                                codes.push(0x8E); 
                                codes.push(0xC0);
                                current_address += 2;

                            },
                            _ => unreachable!()
                        }
                    },
                    Rule::sp => {
                        let mut b = 0x0000;
                        match op2.as_rule() {
                            Rule::imm16 => {
                                let inner = op2.into_inner().next().unwrap();
                                match inner.as_rule() {
                                    Rule::hex16 => {
                                        b = u16::from_str_radix(inner.as_str(), 16).unwrap();

                                    },
                                    Rule::dec16 => {
                                        b = u16::from_str_radix(inner.as_str(), 10).unwrap();
                                    }
                                    _ => unreachable!()
                                }
                            },
                            _ => unreachable!()
                        }
                        codes.push(0xB8 + 0x04); 
                        let b0 = (0x00FF & b) as u8;
                        let b1 = (0xFF00 & b >> 8) as u8;
                        codes.push(b1);
                        codes.push(b0);
                        current_address += 3;
                    },
                    Rule::si => {

                    },
                    Rule::ah => {
                      //  codes.push(0xB0 + 0x04); 
                      //  let b = u8::from_str_radix(op2.as_str(), 16).unwrap();
                      //  codes.push(b);
                      //  current_address += 2;
                    },
                    Rule::bx => {  //16bit
                    //    codes.push(0xB8 + 0x03); 
                    //    let b = u16::from_str_radix(op2.as_str(), 10).unwrap();
                    //    let b0 = (0x00FF & b) as u8;
                    //    let b1 = (0xFF00 & b >> 8) as u8;
                    //    codes.push(b0);
                    //    codes.push(b1);
                    //    current_address += 3;
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
        //check: throw exception if there is a label to which no address is assigned.
    for (_, &address) in labels_map.iter() {
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

