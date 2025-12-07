fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tiles = std::io::read_to_string(std::io::stdin())?.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let area = (0..tiles.len()-1)
        .flat_map(|i| (i+1..tiles.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            (x2.abs_diff(x1)+1)*(y2.abs_diff(y1)+1)
        })
        .max()
        .unwrap();
    println!("{}", area);
    Ok(())
}
