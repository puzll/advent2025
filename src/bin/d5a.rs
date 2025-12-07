fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let mut lines = input.lines();
    let ranges: Vec<_> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            a.parse().unwrap()..=b.parse().unwrap()
        })
        .collect();
    let fresh: i64 = lines
        .map(|line| {
            let id: i64 = line.parse().unwrap();
            ranges
                .iter()
                .any(|r| r.contains(&id)) as i64
        })
        .sum();
    println!("{}", fresh);
    Ok(())
}
