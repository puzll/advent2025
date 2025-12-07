fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let worksheet: Vec<Vec<&str>> = input.lines()
        .map(|line| {
            line.split_ascii_whitespace().collect()
        })
        .collect();
    let ops = worksheet.last().unwrap();
    let mut acc: Vec<i64> = worksheet[0]
        .iter()
        .map(|a| a.parse().unwrap())
        .collect();
    for line in &worksheet[1..worksheet.len()-1] {
        for ((acc, a), &op) in acc
            .iter_mut()
            .zip(line.iter())
            .zip(ops.iter())
        {
            let a = a.parse::<i64>().unwrap();
            match op {
                "*" => *acc *= a,
                _ => *acc += a,
            }
        }
    }
    let sum: i64 = acc.into_iter().sum();
    println!("{}", sum);
    Ok(())
}
