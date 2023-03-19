use std::{str::FromStr, ops::{Add, Mul}, fs, fmt};

fn main() {
    let filename = "inputs/25.txt";
    let content = fs::read_to_string(filename).unwrap();

    let mut result = Snafu::zero();
    for s in content.lines() {
        result = result + s.parse().unwrap();
    }
    println!("You should supply the SNAFU number '{}' to Bob's console.", result);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Snafu{
    digits: [i8; 20],
}

impl Snafu {
    fn zero() -> Self {
        Self{ digits: [0; 20] }
    }

    fn one() -> Self {
        let mut digits = [0; 20];
        digits[0] = 1;
        Self{ digits }
    }

    fn ten() -> Self {
        let mut digits = [0; 20];
        digits[1] = 2;
        Self{ digits }
    }

    fn pow(&self, exp: u32) -> Self {
        let mut result = Snafu::one();
        for _ in 0..exp {
            result = *self * result;
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSnafuError;

impl FromStr for Snafu {
    type Err = ParseSnafuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = [0; 20];
        for (place, c) in s.chars().rev().enumerate() {
            let d = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => return Err(Self::Err{}),
            };
            digits[place] = d;
        }
        Ok(Self{ digits })
    }
}

impl From<i64> for Snafu {
    fn from(source: i64) -> Self {
        let source = source.to_string().chars().map(|c| c.to_string().parse::<i8>().unwrap()).collect::<Vec<i8>>();
        let mut target = Snafu::zero();
        for (place, digit) in source.iter().rev().enumerate() {
            for _ in 0..*digit {
                target = target + Snafu::ten().pow(place as u32);
            }
        }
        target
    }
}

impl Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut digits = [0; 20];
        let mut carry = 0;
        for place in 0..20 {
            let mut d = self.digits[place] + rhs.digits[place] + carry;
            carry = 0;
            if d < -2 {
                carry = -1;
                d += 5;
            } else if d > 2 {
                carry = 1;
                d -= 5;
            }
            digits[place] = d;
        }
        Self::Output{ digits }
    }
}

impl Mul for Snafu {
    type Output = Self;

    // `self` should always be smaller than `rhs`, this keeps the loop
    // iterations as small as possible, becomes very slow otherwise
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Snafu::zero();
        let mut counter = 0;
        let max: i64 = self.into();
        while counter != max {
            result = result + rhs;
            counter += 1;
        }
        result
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for d in self.digits {
            let c = match d {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("invalid digit in Snafu: {}", d),
            };
            s.push(c);
        }
        let mut c = s.pop().unwrap();
        while c == '0' {
            c = s.pop().unwrap();
        }
        s.push(c);
        let s = s.chars().rev().collect::<String>();
        write!(f, "{}", s)
    }
}


impl From<Snafu> for i64 {
    fn from(source: Snafu) -> Self {
        let mut target = 0;
        for (place, digit) in source.digits.iter().enumerate() {
            target += *digit as i64 * 5_i64.pow(place as u32);
        }
        target
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decimal_to_snafu() {
        let content = "0
1
2
3
4
5
6
7
8
9
10
15
20
2022
12345
314159265";

        let expected = "0
1
2
1=
1-
10
11
12
2=
2-
20
1=0
1-0
1=11-2
1-0---0
1121-1110-1=0";

        for (d_str, s_str_expected) in content.lines().zip(expected.lines()) {
            let d = d_str.parse::<i64>().unwrap();
            let s: Snafu = d.into();
            let s_expected = s_str_expected.parse::<Snafu>().unwrap();
            assert_eq!(s, s_expected);
        }
    }

    #[test]
    fn test_snafu_to_decimal() {
        let content = "0
1
2
1=
1-
10
11
12
2=
2-
20
1=0
1-0
1=11-2
1-0---0
1121-1110-1=0";

        let expected = "0
1
2
3
4
5
6
7
8
9
10
15
20
2022
12345
314159265";

        for (s_str, d_str_expected) in content.lines().zip(expected.lines()) {
            let s = s_str.parse::<Snafu>().unwrap();
            let d: i64 = s.into();
            let d_expected = d_str_expected.parse::<i64>().unwrap();
            assert_eq!(d, d_expected);
        }
    }

    #[test]
    fn test_add() {
        let one = "1".parse::<Snafu>().unwrap();
        let two = "2".parse::<Snafu>().unwrap();
        let three = "1=".parse::<Snafu>().unwrap();
        assert_eq!(one + two, three);
    }
}
