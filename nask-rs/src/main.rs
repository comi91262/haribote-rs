#![feature(test)]
extern crate test;

extern crate byteorder;
use std::io::{BufWriter, Write};
use byteorder::{WriteBytesExt, LittleEndian};
use std::io::{Read, BufRead, BufReader, self};
use std::collections::HashMap;

extern crate nask_rs;
use nask_rs::parser::parse;
use nask_rs::parser::Rule;

extern crate pest;

enum Value {
    Byte(u8),
    Label(String, u8),
    NoCode
}

use Value::*;

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
fn exec_inner() {

    let mut codes = vec![];
    let mut labels_map = HashMap::new();

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
                            codes.push(Byte(to_u8(inner.as_str(), inner.as_rule())));
                        },
                        Rule::str => {
                            for b in op.as_str().bytes() {
                                codes.push(Byte(b));
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
                let mut dw = vec![];
                dw.write_u16::<LittleEndian>(b).unwrap();
                let mut wrapped_dw = dw.into_iter().map(|x| Byte(x)).collect();
                codes.append(&mut wrapped_dw);
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
                let mut dd = vec![];
                dd.write_u32::<LittleEndian>(b).unwrap();
                let mut wrapped_dd = dd.into_iter().map(|x| Byte(x)).collect();
                codes.append(&mut wrapped_dd);
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
                                let current_address = current_address + codes.len() as u16;
                                for _ in 0..(n - current_address) {
                                    codes.push(Byte(0)); 
                                }
                            }
                            None    => {
                                for _ in 0..n {
                                    codes.push(Byte(0));
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
                        codes.push(Byte(0xCD));
                        let inner = op.into_inner().next().unwrap();
                        codes.push(Byte(to_u8(inner.as_str(), inner.as_rule())));
                    },
                    _ => unreachable!()
                }
            },
            Rule::jmp => {
                let label = pairs.next().unwrap().as_str();
                codes.push(Byte(0xEB));
                codes.push(Label(label.to_string(), 1));
            },
            Rule::jae => {
                let label = pairs.next().unwrap().as_str();
                codes.push(Byte(0x73));
                codes.push(Label(label.to_string(), 1));
            },
            Rule::jbe => {
                let label = pairs.next().unwrap().as_str();
                codes.push(Byte(0x76));
                codes.push(Label(label.to_string(), 1));
            },
            Rule::jnc => {
                let label = pairs.next().unwrap().as_str();
                codes.push(Byte(0x73));
                codes.push(Label(label.to_string(), 1));
            },
            Rule::je => {
                let label = pairs.next().unwrap().as_str();
                codes.push(Byte(0x74));
                codes.push(Label(label.to_string(), 1));
            },
            Rule::label => {
                let label = operator.as_str();
                labels_map.insert(label.to_string(), codes.len());
            },
            Rule::hlt => {
                codes.push(Byte(0xF4));
            },
            Rule::cmp => {
                let op1 = pairs.next().unwrap();
                let op2 = pairs.next().unwrap();

                match op1.as_rule() {
                    Rule::al => {
                        codes.push(Byte(0x3C)); 
                        let inner = op2.into_inner().next().unwrap();
                        codes.push(Byte(to_u8(inner.as_str(), inner.as_rule())));
                    }
                    Rule::ax => {
                        codes.push(Byte(0x3D)); 
                        panic!("not implemented");

                    }
                    Rule::eax => {
                        codes.push(Byte(0x3D)); 
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
                        codes.push(Byte(0x83)); 
                        codes.push(Byte(0xC6));
                        let inner = op2.into_inner().next().unwrap();
                        codes.push(Byte(to_u8(inner.as_str(), inner.as_rule())));
                    },
                    _ => unreachable!()
                }
            },
            Rule::mov => {
                let op1 = pairs.next().unwrap();
                let op2 = pairs.next().unwrap();
                match op1.as_rule() {
                    Rule::ax => {
                        codes.push(Byte(0xb8)); 
                        let inner = op2.into_inner().next().unwrap();
                        let b = to_u16(inner.as_str(), inner.as_rule());
                        let mut dw = vec![];
                        dw.write_u16::<LittleEndian>(b).unwrap();
                        let mut wrapped_dw = dw.into_iter().map(|x| Byte(x)).collect();
                        codes.append(&mut wrapped_dw);
                    },
                    Rule::ss => {
                        match op2.as_rule() {
                            Rule::ax => {
                                codes.push(Byte(0x8E)); 
                                codes.push(Byte(0xD0));
                            },
                            _ => unreachable!()
                        }
                    },
                    Rule::ds => {
                        match op2.as_rule() {
                            Rule::ax => {
                                codes.push(Byte(0x8E)); 
                                codes.push(Byte(0xD8));
                            },
                            _ => unreachable!()
                        }
                    },
                    Rule::es => {
                        match op2.as_rule() {
                            Rule::ax => {
                                codes.push(Byte(0x8E)); 
                                codes.push(Byte(0xC0));
                            },
                            _ => unreachable!()
                        }
                    },
                    Rule::al => {
                        match op2.as_rule() {
                            Rule::si => {
                                codes.push(Byte(0x8A));
                                codes.push(Byte(0x04)); 
                            },
                            _ => unreachable!()
                        };
                    },
                    Rule::sp => {
                        let b = match op2.as_rule() {
                            Rule::imm16 => {
                                let inner = op2.into_inner().next().unwrap();
                                to_u16(inner.as_str(), inner.as_rule())
                            },
                            _ => unreachable!()
                        };

                        codes.push(Byte(0xB8 + 0x04)); 
                        let mut dw = vec![];
                        dw.write_u16::<LittleEndian>(b).unwrap();
                        let mut wrapped_dw = dw.into_iter().map(|x| Byte(x)).collect();
                        codes.append(&mut wrapped_dw);
                    },
                    Rule::si => {
                        let label = op2.as_str();
                        codes.push(Byte(0xBE));
                        codes.push(Label(label.to_string(), 2));
                        codes.push(NoCode);
                    },
                    Rule::ah => {
                        codes.push(Byte(0xB0 + 0x04)); 
                        let inner = op2.into_inner().next().unwrap();
                        codes.push(Byte(to_u8(inner.as_str(), inner.as_rule())));
                    },
                    Rule::bx => {  //16bit
                        codes.push(Byte(0xB8 + 0x03)); 
                        let inner = op2.into_inner().next().unwrap();
                        let b = to_u16(inner.as_str(), inner.as_rule());
                        let mut dw = vec![];
                        dw.write_u16::<LittleEndian>(b).unwrap();
                        let mut wrapped_dw = dw.into_iter().map(|x| Byte(x)).collect();
                        codes.append(&mut wrapped_dw);
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    for (idx, value) in codes.iter().enumerate() {
        match *value {
            Byte(b) => {
                writer.write_all(&[b]).unwrap();
            },
            Label(ref l, len) => {
                let address = labels_map.get(l).unwrap_or_else(|| {
                    println!("{} has no value.", l);
                    std::process::exit(1);
                });

                if len == 1 {
                    let b = (address - idx - 1) as u16 ;
                    writer.write_all(&[b as u8]).unwrap();
                } else if len == 2 {
                    let b = *address as u16 + current_address;
                    let mut dw = vec![];
                    dw.write_u16::<LittleEndian>(b).unwrap();
                    writer.write_all(&dw).unwrap();
                }
            }
            NoCode => {
            }
        }
    }
}


fn main() {
    exec_inner();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn whole(b: &mut Bencher) {
        b.iter(|| exec_inner());
    }

}
