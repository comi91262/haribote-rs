#![feature(test)]
extern crate test;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use parser::parse;

    #[bench]
    fn bench_db_hex(b: &mut Bencher) {
        b.iter(|| parse("DB 0xaa, 0xfa, 0xa2"));
    }

    #[bench]
    fn bench_db_str(b: &mut Bencher) {
        b.iter(|| parse("DB \"HELLO OS\""));
    }

    #[bench]
    fn bench_db_num(b: &mut Bencher) {
        b.iter(|| parse("DB 1"));
    }

    #[bench]
    fn bench_dw_hex(b: &mut Bencher) {
        b.iter(|| parse("DW 0xabab"));
    }

    #[bench]
    fn bench_dw_num(b: &mut Bencher) {
        b.iter(|| parse("DW 2880"));
    }
    #[bench]
    fn bench_dd_hex(b: &mut Bencher) {

        b.iter(|| parse("DD 0xaaaaaaaa"));
    }
    #[bench]
    fn bench_dd_num(b: &mut Bencher) {
        b.iter(|| parse("DD 2880"));
    }

    #[bench]
    fn bench_resb_num(b: &mut Bencher) {
        b.iter(|| parse("RESB 18"));
    }

    #[bench]
    fn bench_resb_hex(b: &mut Bencher) {
        b.iter(|| parse("RESB 0x01fe-$"));
    }

    #[bench]
    fn bench_empty(b: &mut Bencher) {
        b.iter(|| parse(""));
    }
}
