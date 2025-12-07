fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let mut lines = input.lines();
    let mut beams: Vec<i64> = lines.next().unwrap()
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();
    let mut worlds: i64 = 1;
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if beams[i] > 0 && c == '^' {
                worlds += beams[i];
                if i > 0 { beams[i-1] += beams[i]; }
                if i < beams.len()-1 { beams[i+1] += beams[i]; }
                beams[i] = 0;
            }
        }
    }
    println!("{}", worlds);
    Ok(())
}
