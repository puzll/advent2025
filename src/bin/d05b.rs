fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let mut ranges: Vec<(i64, i64)> = input.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    ranges.sort_unstable();
    let mut ranges = ranges.into_iter();
    let mut merged = vec![ranges.next().unwrap()];
    for range in ranges {
        let last = merged.last_mut().unwrap();
        if last.1+1 < range.0 {
            merged.push(range);
        } else {
            last.1 = last.1.max(range.1);
        }
    }
    let fresh: i64 = merged
        .into_iter()
        .map(|(a, b)| b-a+1)
        .sum();
    println!("{}", fresh);
    Ok(())
}
