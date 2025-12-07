fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let mut lines = input.lines();
    let mut beams: Vec<bool> = lines.next().unwrap()
        .chars()
        .map(|c| c == 'S')
        .collect();
    let mut splits: i32 = 0;
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if beams[i] && c == '^' {
                splits += 1;
                beams[i] = false;
                if i > 0 { beams[i-1] = true; }
                if i < beams.len()-1 { beams[i+1] = true; }
            }
        }
    }
    println!("{}", splits);
    Ok(())
}
