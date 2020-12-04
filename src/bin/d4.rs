fn main() {
    let content = std::fs::read_to_string("assets/d4.txt").unwrap();
    let passports = content.split("\n\n").collect::<Vec<_>>();

    let check_year = |inp: &str, min, max| {
        let y = inp.parse::<u16>().unwrap();
        min <= y && y <= max
    };
    let check_height = |inp: &str| {
        let inp = inp.as_bytes();
        let len = inp.len();
        if len == 4 {
            if inp[2..4] == *b"in" {
                let val = inp[0..2]
                    .iter()
                    .fold(0, |sum, curr| sum * 10 + *curr as u16 - 48);
                if 59 <= val && val <= 76 {
                    return true;
                }
            }
        } else if len == 5 {
            if inp[3..5] == *b"cm" {
                let val = inp[0..3]
                    .iter()
                    .fold(0, |sum, curr| sum * 10 + *curr as u16 - 48);
                if 150 <= val && val <= 193 {
                    return true;
                }
            }
        }
        false
    };
    let check_hcl = |inp: &str| {
        let inp = inp.as_bytes();
        if inp.len() == 7 && inp[0] == b'#' {
            return inp[1..]
                .iter()
                .all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'));
        }
        false
    };
    let mut count_1 = 0;
    let mut count_2 = 0;
    for passport in passports {
        let mut valid = [None; 8];
        passport.split_ascii_whitespace().for_each(|kv| {
            let mut kv = kv.split(':');
            let key = kv.next().unwrap();
            let val = kv.next().unwrap();
            let (idx, validity) = match key {
                "cid" => (0, true),
                "byr" => (1, check_year(val, 1920, 2002)),
                "iyr" => (2, check_year(val, 2010, 2020)),
                "eyr" => (3, check_year(val, 2020, 2030)),
                "hgt" => (4, check_height(val)),
                "hcl" => (5, check_hcl(val)),
                "ecl" => (
                    6,
                    matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                ),
                "pid" => (7, val.len() == 9 && val.parse::<u64>().is_ok()),
                _ => unreachable!(),
            };
            valid[idx] = Some(validity);
        });
        if valid.iter().skip(1).all(|x| x.is_some()) {
            count_1 += 1;
        }
        if valid.iter().skip(1).all(|x| x.is_some() && x.unwrap()) {
            count_2 += 1;
        }
    }

    // ans 1
    println!("{:?}", count_1);
    // ans 2
    println!("{:?}", count_2);
}
