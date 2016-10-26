#![cfg_attr(feature = "unstable", feature(test))]
#![cfg(all(feature = "unstable", test))]

extern crate test;

#[bench]
fn a_bench(b: &mut test::Bencher)
{
    b.iter(|| { /* some bench */});
}
