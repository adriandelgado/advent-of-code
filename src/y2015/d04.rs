use md5::{Digest, Md5};

pub(super) fn part1(input: &str) -> String {
    let input = input.trim_end();
    for n in 0.. {
        if verify_hash_5(format!("{input}{n}").as_bytes()) {
            return n.to_string();
        }
    }
    unreachable!()
}

pub(super) fn part2(input: &str) -> String {
    let input = input.trim_end();
    for n in 0.. {
        if verify_hash_6(format!("{input}{n}").as_bytes()) {
            return n.to_string();
        }
    }
    unreachable!()
}

fn verify_hash_5(input: &[u8]) -> bool {
    let mut hasher = Md5::new();

    hasher.update(input);

    let [n1, n2, n3, ..]: [u8; 16] = hasher.finalize().into();

    n1 == 0 && n2 == 0 && n3 < 16
}

fn verify_hash_6(input: &[u8]) -> bool {
    let mut hasher = Md5::new();

    hasher.update(input);

    let [n1, n2, n3, ..]: [u8; 16] = hasher.finalize().into();

    n1 == 0 && n2 == 0 && n3 == 0
}
