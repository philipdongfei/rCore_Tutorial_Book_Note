#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

static mut S: [u64; LEN] = [0u64; LEN];

#[no_mangle]
fn main() -> i32 {
    println!("Test power_3 begin:");
    let p = 3u64;
    let m = 998244353u64;
    let iter: usize = 300000;
    let mut cur = 0usize;
    unsafe {
        S[cur] = 1;
    }
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        unsafe {
            S[next] = S[cur] * p % m;

        }
        cur = next;
        if i % 10000 == 0 {
            println!("power_3 [{}/{}]", i, iter);
        }
    }
    unsafe {
        println!("{}^{} = {}(MOD {})", p, iter, S[cur], m);
    }
    println!("Test power_3 OK!");
    0
}
