#![feature(asm)]
#![feature(test)]

use rand::SeedableRng;
extern crate rand;

extern crate time;

use test::test::Bencher;
extern crate test;

fn rand_int() -> Option<u64> {
    let rand_int: u64;
    let ok: u8;
    unsafe {
        asm!("rdrand $0; setc $1;"
             :"=r"(rand_int), "=r"(ok)
             :
             :
             );
    }
    if ok == 0x01 {
        Some(rand_int)
    } else {
        None
    }
}

fn fast_rand_int() -> u64 {
    let rand_int: u64;
    unsafe {
        asm!("rdrand $0;"
             :"=r"(rand_int)
             :
             :
             );
    }
    rand_int
}

fn seeded_rand_int(rng: &mut rand::Rng) -> u64 {
    return rng.next_u64();
}

#[bench]
fn bench_hwrand_int(b: &mut Bencher) {
    b.iter(|| rand_int());
}

#[bench]
fn bench_fast_hwrand_int(b: &mut Bencher) {
    b.iter(|| fast_rand_int());
}

#[bench]
fn bench_seededrand_int(b: &mut Bencher) {
    let millis = time::precise_time_ns() as usize;
    let mut rng = rand::StdRng::from_seed(&[millis]);
    b.iter(|| seeded_rand_int(&mut rng));
}

fn main() {

    let millis = time::precise_time_ns() as usize;
    let mut rng = rand::StdRng::from_seed(&[millis]);

    println!("Hello rng: {:?}", rand_int());
    println!("Hello: {}", 0x00 & 0x01);
    println!("Hello seeded rng: {:?}", seeded_rand_int(&mut rng));
}
