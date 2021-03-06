#![feature(test)]

extern crate test;
extern crate gameoflife;

use test::Bencher;
use gameoflife::map;

#[bench]
fn benchmark_with_acorn(b: &mut Bencher) {
    let mut map = map::Map::acorn();
    for _ in 0..4408 {
        map.next_generation();
    }
    b.iter(|| map.clone().next_generation());
}
