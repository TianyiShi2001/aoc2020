fn main() {
    let content = std::fs::read_to_string("assets/d5.txt").unwrap();
    let content = content.lines().collect::<Vec<_>>();
    let mut mx = 0;
    let mut seats = Vec::new();
    for &line in &content {
        let line = line.as_bytes();
        let mut l = 0;
        let mut r = 128;
        let mut m = (l + r) / 2;
        for i in 0..7 {
            match line[i] {
                b'F' => r = m,
                b'B' => l = m,
                _ => unreachable!(),
            }
            m = (l + r) / 2;
        }
        let row = m;
        l = 0;
        r = 8;
        m = (l + r) / 2;
        for j in 7..10 {
            match line[j] {
                b'L' => r = m,
                b'R' => l = m,
                _ => unreachable!(),
            }
            m = (l + r) / 2;
        }
        let col = m;
        let id = row * 8 + col;
        if id > mx {
            mx = id;
        }
        seats.push([row, col]);
    }
    seats.sort();
    // q1
    println!("{:?}", mx);
    let mut expected = 0;
    for pair in &seats[1..seats.len() - 4] {
        if expected != pair[1] {
            // q2
            println!("{:?}", [pair[0], expected]);
            break;
        }
        expected = (expected + 1) % 8;
    }
}
