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

fn to_u8(imm: &str, rule: Rule) -> u8 {
    match rule {
        Rule::hex8 => {
            let (first, last) = imm.split_at(2);
            if first == "0x" {
                u8::from_str_radix(last, 16).unwrap()
            } else {
                panic!("to_u8");
            }
        },
        Rule::dec8 => {
            match u8::from_str_radix(imm, 10) {
                Ok(value) => value,
                Err(e) => panic!("overflow : {:?}", e)
            }
        },
        _ => unreachable!() 
    }
}

fn to_u16(imm: &str, rule: Rule) -> u16 {
    match rule {
        Rule::hex16 => {
            let (first, last) = imm.split_at(2);
            if first == "0x" {
                u16::from_str_radix(last, 16).unwrap()
            } else {
                panic!("to_u16");
            }
        },
        Rule::dec16 => {
            match u16::from_str_radix(imm, 10) {
                Ok(value) => value,
                Err(e) => panic!("overflow : {:?}", e)
            }
        },
        _ => unreachable!() 
    }
}

fn to_u32(imm: &str, rule: Rule) -> u32 {
    match rule {
        Rule::hex32 => {
            let (first, last) = imm.split_at(2);
            if first == "0x" {
                u32::from_str_radix(last, 16).unwrap()
            } else {
                panic!("to_u32");
            }
        },
        Rule::dec32 => {
            match u32::from_str_radix(imm, 10) {
                Ok(value) => value,
                Err(e) => panic!("overflow : {:?}", e)
            }
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
                let op = pairs.next().unwrap();
                let inner = op.into_inner().next().unwrap();
                current_address += to_u16(inner.as_str(), inner.as_rule());
            },
            Rule::db => {
                for op in pairs {
                    match op.as_rule() {
                        Rule::imm8 => {
                            let inner = op.into_inner().next().unwrap();
                            codes.push(to_u8(inner.as_str(), inner.as_rule()));
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
                let op = pairs.next().unwrap();
                let b = match op.as_rule() {
                    Rule::imm16 => {
                        let inner = op.into_inner().next().unwrap();
                        to_u16(inner.as_str(), inner.as_rule())
                    },
                    _ => unreachable!()
                };
                let b10 =  b & 0x00FF;
                let b32 = (b & 0xFF00) >> 8;
                codes.push(b10 as u8);
                codes.push(b32 as u8);
                current_address += 2;
            },
            Rule::dd => {
                let op = pairs.next().unwrap();
                let b = match op.as_rule() {
                    Rule::imm32 => {
                        let inner = op.into_inner().next().unwrap();
                        to_u32(inner.as_str(), inner.as_rule())
                    },
                    _ => unreachable!()
                };

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
                let op     = pairs.next().unwrap();
                let option = pairs.next();
                match op.as_rule() {
                    Rule::imm16 => {
                        let inner = op.into_inner().next().unwrap();
                        let n = to_u16(inner.as_str(), inner.as_rule());

                        match option {
                            Some(_) => {
                                for _ in 0..(n - current_address) {
                                    codes.push(0); 
                                    current_address += 1;
                                }
                            }
                            None    => {
                                for _ in 0..n {
                                    codes.push(0);
                                    current_address += 1;
                                }
                            }
                        }
                    }
                    _ => unreachable!()
                }
            },
            Rule::int => {
                let op = pairs.next().unwrap();
                match op.as_rule() {
                    Rule::imm8 => {
                        codes.push(0xCD);
                        let inner = op.into_inner().next().unwrap();
                        codes.push(to_u8(inner.as_str(), inner.as_rule()));
                        current_address += 2;
                    },
                    _ => unreachable!()
                }
            },
            Rule::jmp => {
                let label = pairs.next().unwrap().as_str();
                codes.push(0xEB);
                codes.push(0xFF); //tmp
                address_map.insert((current_address+1) as u32, label.to_string());
                current_address += 2;
            },
            Rule::je => {
                let label = pairs.next().unwrap().as_str();
                codes.push(0x74);
                codes.push(0xFE);  //tmp
                address_map.insert((current_address+1) as u32, label.to_string());
                current_address += 2;
            },
            Rule::label => {
                let label = operator.as_str();
                labels_map.insert(label.to_string(), current_address);
            },
            Rule::hlt => {
                codes.push(0xF4);
                current_address += 1;
            },
            Rule::cmp => {
                let op1 = pairs.next().unwrap();
                let op2 = pairs.next().unwrap();

                match op1.as_rule() {
                    Rule::al => {
                        codes.push(0x3C); 
                        let inner = op2.into_inner().next().unwrap();
                        codes.push(to_u8(inner.as_str(), inner.as_rule()));
                        current_address += 2;
                    }
                    Rule::ax => {
                        codes.push(0x3D); 
                        panic!("not implemented");
                        
                    }
                    Rule::eax => {
                        codes.push(0x3D); 
                        panic!("not implemented");
                    }
                    _ => unreachable!()
                }
            },
            Rule::add => {
                let op1 = pairs.next().unwrap();
                let op2 = pairs.next().unwrap();

                //TODO table
                match op1.as_rule() {
                    Rule::si => {
                            codes.push(0x83); 
                            codes.push(0xC6);
                            let inner = op2.into_inner().next().unwrap();
                            codes.push(to_u8(inner.as_str(), inner.as_rule()));
                            current_address += 3;
                    },
                    _ => unreachable!()
                }
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
                        let b = match op2.as_rule() {
                            Rule::imm16 => {
                                let inner = op2.into_inner().next().unwrap();
                                to_u16(inner.as_str(), inner.as_rule())
                            },
                            _ => unreachable!()
                        };

                        codes.push(0xB8 + 0x04); 
                        let b0 =  0x00FF & b;
                        let b1 = (0xFF00 & b) >> 8;
                        codes.push(b0 as u8);
                        codes.push(b1 as u8);
                        current_address += 3;
                    },
                    Rule::si => {

                    },
                    Rule::ah => {
                        codes.push(0xB0 + 0x04); 
                        let inner = op2.into_inner().next().unwrap();
                        codes.push(to_u8(inner.as_str(), inner.as_rule()));
                        current_address += 2;
                    },
                    Rule::bx => {  //16bit
                        codes.push(0xB8 + 0x03); 
                        let inner = op2.into_inner().next().unwrap();
                        let b = to_u16(inner.as_str(), inner.as_rule());
                        let b0 = (0x00FF & b) as u8;
                        let b1 = (0xFF00 & b >> 8) as u8;
                        codes.push(b0);
                        codes.push(b1);
                        current_address += 3;
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
        //JMP
        if b == 0xFF {
            match address_map.get(&((idx as u32) + 0x7c00)) {
                Some(label) => {
                    let address = labels_map.get(label).unwrap();
                    let bb = 0x00FF & address;
                    writer.write_fmt(format_args!(r"\x{:02X}", &(bb - 2))).unwrap(); //-2 ??
                },
                None => writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap()
            }
        //JE
        } else if b == 0xFE {
            match address_map.get(&((idx as u32) + 0x7c00)) {
                Some(label) => {
                    let address = labels_map.get(label).unwrap();
                    let bb = 0x00FF & address - (idx as u16) + 1;
                    writer.write_fmt(format_args!(r"\x{:02X}", &(bb - 2))).unwrap(); //-2 ??
                },
                None => writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap()
            }
        } else {
            writer.write_fmt(format_args!(r"\x{:02X}", b)).unwrap();
        }
    }


}

