use std::fs::File;
use std::io::*;
#[derive(Debug)]
struct Case {
    a: usize,
    b: usize,
    letter: char,
    password: String,
}

impl Case {
    fn parse(inp: &str) -> Self {
        let mut stmt = inp.split_ascii_whitespace();
        let mut ab = stmt
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<usize>().unwrap());
        let a = ab.next().unwrap();
        let b = ab.next().unwrap();
        let letter = stmt.next().unwrap().chars().next().unwrap();
        Self {
            a,
            b,
            letter,
            password: stmt.next().unwrap().to_owned(),
        }
    }
    fn is_valid_1(&self) -> bool {
        let count = self.password.matches(self.letter).count();
        self.a <= count && count <= self.b
    }
    fn is_valid_2(&self) -> bool {
        let passwd = self.password.as_bytes();
        if let Some(c) = passwd.get(self.a - 1) {
            if let Some(d) = passwd.get(self.b - 1) {
                if (*c as char == self.letter) ^ (*d as char == self.letter) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let inp = File::open("assets/d2.txt").unwrap();
    let rdr = BufReader::new(inp);

    let cases: Vec<Case> = rdr.lines().map(|l| Case::parse(&l.unwrap())).collect();

    let ans1 = cases.iter().filter(|x| x.is_valid_1()).count();
    println!("{}", ans1);

    let ans2 = cases.iter().filter(|x| x.is_valid_2()).count();
    println!("{}", ans2);
    // println!("{}", sol2(&nums).unwrap());
}
