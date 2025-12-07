#[derive(Debug)]
struct Rect {
    top: i64,
    bottom: i64,
    left: i64,
    right: i64,
}

impl Rect {
    fn new((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> Self {
        Rect {
            top: y1.max(y2),
            bottom: y1.min(y2),
            left: x1.min(x2),
            right: x1.max(x2),
        }
    }

    fn inside(&self, (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> bool {
        if y1 == y2 {
            (self.bottom+1..self.top).contains(&y1) &&
            x1.min(x2) < self.right &&
            x1.max(x2) > self.left
        } else {
            (self.left+1..self.right).contains(&x1) &&
            y1.min(y2) > self.top &&
            y1.max(y2) < self.bottom
        }
    }

    fn cross(&self, (x1, y1): (i64, i64), (x2, _): (i64, i64)) -> i64 {
        if y1 < self.top {
            0
        } else if (x1..x2).contains(&self.left) {
            -1
        } else if (x2..x1).contains(&self.left) {
            1
        } else {
            0
        }
    }

    fn area(&self) -> i64 {
        (self.top-self.bottom+1) * (self.right-self.left+1)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tiles = std::io::read_to_string(std::io::stdin())?.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let area = (0..tiles.len()-1)
        .flat_map(|i| (i+1..tiles.len()).map(move |j| (i, j)))
        .map(|(i, j)| Rect::new(tiles[i], tiles[j]))
        .filter(|rect| {
            let mut loops = 0;
            for v in 0..tiles.len() {
                let u = (v+1) % tiles.len();
                if rect.inside(tiles[v], tiles[u]) {
                    return false;
                }
                loops += rect.cross(tiles[v], tiles[u]);
            }
            loops%2 != 0
        })
        .map(|rect| rect.area())
        .max()
        .unwrap();
    println!("{}", area);
    Ok(())
}
