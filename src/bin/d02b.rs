fn invalid(id: &[u8]) -> bool {
    let size = id.len();
    'l: for m in 1..=size/2 {
        if m >= size || size%m != 0 { continue 'l; }
        let mut chunks = id.chunks(m);
        let first = chunks.next().unwrap();
        for chunk in chunks {
            if chunk != first { continue 'l; }
        }
        return true;
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum: i64 = 0;
    for range in std::io::read_to_string(std::io::stdin())?.split(',') {
        let (begin, end) = range.split_once('-').ok_or("Invalid range")?;
        let begin = begin.parse::<i64>()?;
        let end = end.trim_end().parse::<i64>()?;
        for id in begin..=end {
            if invalid(id.to_string().as_bytes()) {
                sum += id;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
