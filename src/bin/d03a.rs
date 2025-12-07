fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum: i32 = 0;
    for line in std::io::read_to_string(std::io::stdin())?.lines() {
        let line = line.as_bytes();
        let (i, first) = line[0..line.len()-1]
            .iter()
            .enumerate()
            .max_by(|(i1, v1), (i2, v2)| {
                let r = v1.cmp(v2);
                if r != std::cmp::Ordering::Equal { r } else { i2.cmp(i1) }
            })
            .unwrap();
        let second = line[i+1..line.len()]
            .iter()
            .max()
            .unwrap();
        sum += ((first-b'0')*10 + second-b'0') as i32;
    }
    println!("{}", sum);
    Ok(())
}
