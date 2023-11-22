pub fn part1(input: &str) -> String {
    let mut pass = Password::from(input.trim_end());
    loop {
        pass.increment();

        if pass.is_valid() {
            break pass.into();
        }
    }
}

pub fn part2(input: &str) -> String {
    part1(&part1(input))
}

#[derive(Clone, Copy)]
struct Password(u64);

impl Password {
    fn increment(&mut self) {
        self.0 += 1;
    }

    fn is_valid(self) -> bool {
        let pass: [u8; 8] = self.into();
        let mut iter = pass.windows(2);

        iter.any(|pair| pair[0] == pair[1])
            && iter.next().is_some()
            && iter.any(|pair| pair[0] == pair[1])
            && pass
                .windows(3)
                .any(|triplet| triplet[0] + 1 == triplet[1] && triplet[0] + 2 == triplet[2])
    }
}

impl From<&str> for Password {
    fn from(pass: &str) -> Self {
        let out = pass
            .chars()
            .rev()
            .zip(0..)
            .fold(0, |acc, (c, i)| acc + char_to_digit(c) * 23_u64.pow(i));

        Self(out)
    }
}

impl From<Password> for [u8; 8] {
    fn from(Password(mut pass): Password) -> Self {
        let mut pass_arr = [0; 8];
        let mut position = 1;
        while pass > 0 {
            let byte = digit_to_byte(pass % 23);
            pass_arr[pass_arr.len() - position] = byte;
            position += 1;
            pass /= 23;
        }

        pass_arr
    }
}

impl From<Password> for String {
    fn from(pass: Password) -> Self {
        let pass: [u8; 8] = pass.into();
        // SAFETY: `digit_to_byte` only outputs ascii
        unsafe { std::str::from_utf8_unchecked(&pass).to_owned() }
    }
}

#[inline]
fn char_to_digit(ch: char) -> u64 {
    let offset = match ch {
        'a'..='h' => 97,
        'j' | 'k' => 98,
        'm' | 'n' => 99,
        _ => 100,
    };

    ch as u64 - offset
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
fn digit_to_byte(d: u64) -> u8 {
    let offset = match d {
        0..=7 => 97,
        8 | 9 => 98,
        10 | 11 => 99,
        _ => 100,
    };

    (d + offset) as u8
}
