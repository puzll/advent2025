fn main() -> Result<(), Box<dyn std::error::Error>> {
    let junctions = std::io::read_to_string(std::io::stdin())?.lines()
        .map(|line| {
            let mut coords = line.split(',');
            (
                coords.next().unwrap().parse::<f64>().unwrap(),
                coords.next().unwrap().parse::<f64>().unwrap(),
                coords.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<(f64, f64, f64)>>();
    let mut cables =
        (0..junctions.len()).flat_map(|i| {
            (0..i).map(move |j| (i, j))
                .map(|(i, j)| {
                    let (x1, y1, z1) = junctions[i];
                    let (x2, y2, z2) = junctions[j];
                    (i, j, ((x2-x1).powi(2) + (y2-y1).powi(2) + (z2-z1).powi(2)).sqrt())
                })
        })
        .collect::<Vec<(usize, usize, f64)>>();
    cables.sort_unstable_by(|j1, j2| j1.2.total_cmp(&j2.2));
    let mut uf = petgraph::unionfind::UnionFind::new(junctions.len());
    let mut n = 0;
    for (a, b, _) in cables {
        if uf.union(a, b) {
            n += 1;
            if n == junctions.len()-1 {
                println!("{}", junctions[a].0*junctions[b].0);
                break;
            }
        };
    }
    Ok(())
}
