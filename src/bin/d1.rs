use std::fs::File;
use std::io::*;

fn main() {
    let inp = File::open("assets/d1.txt").unwrap();
    let rdr = BufReader::new(inp);

    let nums: Vec<usize> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let ans1 = sol1(&nums);
    println!("{}", ans1.unwrap());

    println!("{}", sol2(&nums).unwrap());
}

fn sol1(nums: &[usize]) -> Option<usize> {
    let mut present = [false; 10000];
    for &num in nums {
        let complement = 2020 - num;
        if present[complement] {
            return Some(num * complement);
        }
        present[num] = true;
    }
    None
}

fn sol2(nums: &[usize]) -> Option<usize> {
    let l = nums.len();
    for i in 0..l {
        for j in i + 1..l {
            for k in j + 1..l {
                let (x, y, z) = (nums[i], nums[j], nums[k]);
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}
