#![feature(test)]

extern crate test;

#[bench]
fn bench_naive(b: &mut test::Bencher) {
    b.iter(|| 1+1)
}