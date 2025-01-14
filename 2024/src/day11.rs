use crate::Input;
use core::iter;
use rustc_hash::FxHashMap;

pub fn inputs() -> Vec<Input<u64>> {
    vec![
        Input::Hashed("4fef1a2186d9f5b05be6743b65f574c6519afc4cf61aba34b53408031efe1131"),
        Input::Inline("example", "125 17\n", Some(55312), None),
    ]
}

fn nums(input: &str) -> impl Iterator<Item = u64> + '_ {
    let mut bytes = input.as_bytes().iter();
    iter::from_fn(move || {
        if let Some(b) = bytes.next() {
            let mut num = (b - b'0') as u64;
            for &b in bytes.by_ref() {
                if b < b'0' {
                    break;
                }
                num = num * 10 + (b - b'0') as u64
            }
            Some(num)
        } else {
            None
        }
    })
}

fn split(memo: &mut FxHashMap<(u64, u32), u64>, mut n: u64, mut i: u32) -> u64 {
    if i == 0 {
        return 1;
    } else if let Some(length) = memo.get(&(n, i)) {
        return *length;
    }
    let mut length = 1;
    let (n0, i0) = (n, i);
    while i != 0 {
        i -= 1;
        if n == 0 {
            n = 1
        } else {
            let nd = n.ilog10();
            if nd & 1 == 1 {
                let p = 10_u64.pow((nd + 1) / 2);
                let (hi, lo) = (n / p, n % p);
                length = split(memo, hi, i) + split(memo, lo, i);
                break;
            } else {
                n *= 2024
            }
        }
    }
    memo.insert((n0, i0), length);
    length
}

pub fn part1(input: &str) -> u64 {
    let mut memo = FxHashMap::default();
    nums(input).map(move |n| split(&mut memo, n, 25)).sum()
}

pub fn part2(input: &str) -> u64 {
    let mut memo = FxHashMap::default();
    nums(input).map(move |n| split(&mut memo, n, 75)).sum()
}
