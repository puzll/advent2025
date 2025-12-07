use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut num: i32 = 50;
    let mut count: i32 = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let mut chars = line.chars();
        let dir = chars.next().ok_or("Empty line")?;
        let amount = chars.as_str().parse::<i32>()?;
        match dir {
            'L' => {
                count += (amount - num + 100).div_euclid(100);
                if num == 0 { count -= 1; }
                num = (num - amount).rem_euclid(100);
            },
            'R' => {
                count += (amount + num).div_euclid(100);
                num = (num + amount).rem_euclid(100);
            },
            _ => return Err("Invalid line".into()),
        }
    }
    println!("{}", count);
    Ok(())
}
