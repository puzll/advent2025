fn parse_brackets(s: &str, left: char, right: char) -> Option<(&str, &str)> {
    let s = s.trim_start().strip_prefix(left)?;
    s.split_once(right)
}

fn parse_line(s: &str) -> Option<(u64, Vec<u64>)> {
    let (lights, s) = parse_brackets(s, '[', ']')?;
    let lights = lights
        .chars()
        .enumerate()
        .map(|(i, c)| ((c == '#') as u64) << i)
        .sum();
    let mut wirings = Vec::new();
    let mut s = s;
    while let Some((wiring, r)) = parse_brackets(s, '(', ')') {
        wirings.push(wiring.split(',')
            .map(|s| 1<<s.parse::<usize>().unwrap())
            .sum()
        );
        s = r;
    }
    Some((lights, wirings))
}

fn solve(lights: u64, wirings: &[u64]) -> u64 {
    let mut queue = std::collections::VecDeque::from([(0u64, 0u64, 0usize)]);
    while let Some((state, res, pos)) = queue.pop_front() {
        for i in pos..wirings.len() {
            let bit = 1<<i;
            if state&bit == 0 {
                let res = res^wirings[i];
                if res == lights { return state|bit; }
                queue.push_back((state|bit, res, i+1));
            }
        }
    }
    panic!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buttons = std::io::read_to_string(std::io::stdin())?.lines()
        .map(|line| {
            let (lights, wirings) = parse_line(line).unwrap();
            solve(lights, &wirings).count_ones()
        })
        .sum::<u32>();
    println!("{}", buttons);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kek() {
        let res = solve(0b011101, &[0b111110, 0b100110, 0b111011, 0b011000]);
        assert_eq!(res, 0b110);
    }
}
