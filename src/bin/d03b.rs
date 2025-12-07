fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum: i64 = 0;
    for line in std::io::read_to_string(std::io::stdin())?.lines() {
        let line = line.as_bytes();
        let mut i = 0;
        let mut r = 12;
        let mut joltage = 0;
        while r > 0 {
            r -= 1;
            let (j, digit) = line[i..line.len()-r]
                .iter()
                .enumerate()
                .max_by(|(i1, v1), (i2, v2)| {
                    let r = v1.cmp(v2);
                    if r != std::cmp::Ordering::Equal { r } else { i2.cmp(i1) }
                })
                .unwrap();
            i += j+1;
            joltage *= 10;
            joltage += (digit-b'0') as i64;
        }
        sum += joltage;
    }
    println!("{}", sum);
    Ok(())
}
