use itertools::Itertools;

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
    for i in 0..1000 {
        let (a, b, _) = cables[i];
        uf.union(a, b);
    }
    let mut circuits = (0..junctions.len())
        .map(|i| uf.find_mut(i))
        .collect::<Vec<usize>>();
    circuits.sort_unstable();
    let mut sizes = circuits
        .iter()
        .dedup_with_count()
        .map(|e| e.0)
        .collect::<Vec<usize>>();
    sizes.select_nth_unstable_by_key(2, |&e| std::cmp::Reverse(e));
    let product = sizes[0..=2]
        .iter()
        .product::<usize>();
    println!("{}", product);
    Ok(())
}
