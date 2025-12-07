fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let mut lines = input.lines().peekable();
    while lines.peek().unwrap().strip_suffix(':').is_some() {
        lines.next();
        lines.next();
        lines.next();
        lines.next();
        lines.next();
    }
    let regions = lines
        .filter(|line| {
            let (h, tail) = line.split_once('x').unwrap();
            let (w, tail) = tail.split_once(':').unwrap();
            let spaces = (h.parse::<i32>().unwrap()/3) * (w.parse::<i32>().unwrap()/3);
            let presents = tail.trim_start().split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .sum();
            spaces >= presents
        })
        .count();
    println!("{}", regions);
    Ok(())
}
