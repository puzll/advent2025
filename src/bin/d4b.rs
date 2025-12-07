struct Grid {
    grid: Vec<Vec<bool>>,
    n: usize,
    m: usize,
}

impl Grid {
    fn new(s: &str) -> Grid {
        let grid: Vec<Vec<bool>> = s.lines()
            .map(|line| line
                .chars()
                .map(|c| c == '@')
                .collect()
            )
            .collect();
        let n = grid.len();
        let m = grid[0].len();
        Grid { grid, n, m }
    }

    fn get(&self, i: i32, j: i32) -> i32 {
        if i >= 0 && i < self.n as i32 && j >= 0 && j < self.m as i32 {
            self.grid[i as usize][j as usize] as i32
        } else {
            0
        }
    }

    fn has_access(&self, i: usize, j: usize) -> bool {
        let i = i as i32;
        let j = j as i32;
        self.get(i, j) > 0 &&
        self.get(i, j+1) +
        self.get(i-1, j+1) +
        self.get(i-1, j) +
        self.get(i-1, j-1) +
        self.get(i, j-1) +
        self.get(i+1, j-1) +
        self.get(i+1, j) +
        self.get(i+1, j+1) < 4
    }

    fn remove(&mut self) -> i32 {
        let mut removed = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                if self.has_access(i, j) {
                    self.grid[i][j] = false;
                    removed += 1;
                }
            }
        }
        removed
    }

    fn remove_all(&mut self) -> i32 {
        std::iter::from_fn(|| {
            let r = self.remove();
            if r > 0 { Some(r) } else { None }
        })
        .sum()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = Grid::new(&std::io::read_to_string(std::io::stdin())?);
    println!("{}", grid.remove_all());
    Ok(())
}
