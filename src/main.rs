use core::panic;

fn main() {
    square(0);
}

pub fn square(s: u32) -> u64 {
    let s = s as usize;
    let mut result = Vec::new();

    for i in 0..s {
        if i >= 64 {
            panic!("Square must be between 1 and 64");
        }
        result.push(u64::pow(2, i as u32));
    }

    if result.is_empty() {
        panic!("Square must be between 1 and 64");
    } else {
        result[s - 1]
    }
}

pub fn total() -> u64 {
    let mut result = Vec::new();

    for i in 0..64 as u64 {
        result.push(u64::pow(2, i.try_into().unwrap()));
    }

    result.iter().map(|&i| i as u64).sum()
}
