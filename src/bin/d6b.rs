fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let worksheet: Vec<&[u8]> = input.lines()
        .map(|line| line.as_bytes())
        .collect();
    let mut op = b' ';
    let mut sum = 0;
    let mut res = 0;
    for j in 0..worksheet[0].len() {
        let mut num = 0;
        for i in 0..worksheet.len()-1 {
            let digit = worksheet[i][j];
            if digit != b' ' {
                num *= 10;
                num += (digit-b'0') as i64;
            }
        }
        let bottom = worksheet[worksheet.len()-1][j];
        if bottom != b' ' {
            op = bottom;
            res = num;
        } else if num != 0 {
            match op {
                b'+' => res += num,
                _ => res *= num,
            }
        } else {
            sum += res;
        }
    }
    sum += res;
    println!("{}", sum);
    Ok(())
}
