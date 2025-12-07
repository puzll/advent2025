use ndarray::*;
use num_rational::Rational32;

fn parse_brackets(s: &str, left: char, right: char) -> Option<(&str, &str)> {
    let s = s.trim_start().strip_prefix(left)?;
    s.split_once(right)
}

fn parse_line(s: &str) -> Option<(Vec<Vec<usize>>, Vec<i32>)> {
    let (_, s) = parse_brackets(s, '[', ']')?;
    let mut wirings = Vec::new();
    let mut s = s;
    while let Some((wiring, r)) = parse_brackets(s, '(', ')') {
        wirings.push(wiring.split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        );
        s = r;
    }
    wirings.sort_unstable_by_key(|button| std::cmp::Reverse(button.len()));
    let joltages = parse_brackets(s, '{', '}')?.0.split(',')
        .map(|joltage| joltage.parse().unwrap())
        .collect::<Vec<_>>();
    Some((wirings, joltages))
}

struct Machine {
    a: Array2<Rational32>,
    x: Array1<Rational32>,
    max_xs: Vec<i32>,
    free: usize,
}

impl Machine {
    fn new(line: &str) -> Self {
        let (wirings, joltages) = parse_line(line).unwrap();
        let mut a = Array2::zeros((joltages.len(), wirings.len()+1));
        let mut max_xs = Vec::new();
        for (j, wiring) in wirings.iter().enumerate() {
            for &i in wiring {
                a[(i, j)] = Rational32::ONE;
            }
            max_xs.push(wiring.iter().map(|&i| joltages[i]).min().unwrap());
        }
        for (i, joltage) in joltages.into_iter().enumerate() {
            a[(i, wirings.len())] = joltage.into();
        }
        Machine {
            a,
            x: Array1::zeros(wirings.len()),
            max_xs, 
            free: 0,
        }
    }

    fn swap_rows(&mut self, i: usize, j: usize) {
        for mut column in self.a.columns_mut() {
            column.swap(i, j);
        }
    }

    fn swap_columns(&mut self, i: usize, j: usize) {
        for mut row in self.a.rows_mut() {
            row.swap(i, j);
        }
        self.max_xs.swap(i, j);
    }

    fn gauss(&mut self) {
        let mut i = 0;
        while i < self.a.nrows() && i < self.x.len() {
            if let Some(((jj, ii), _)) = self.a.slice(s![i.., i..])
                .reversed_axes()
                .into_indexed_iter()
                .find(|&(_, &e)| e != Rational32::ZERO)
            {
                self.swap_rows(i, i+ii);
                self.swap_columns(i, i+jj);
                let mut rows = self.a.slice_mut(s![i.., ..]).into_axis_iter_mut(Axis(0));
                let first = rows.next().unwrap();
                for row in rows {
                    let k = row[i]/first[i];
                    Zip::from(&first).and(row).for_each(|&a, b| *b -= k*a);
                }
                i += 1;
            } else {
                break;
            }
        }
        self.free = i;
        while i > 0 {
            let mut rows = self.a.slice_mut(s![0..i, ..]).into_axis_iter_mut(Axis(0)).rev();
            i -= 1;
            let mut last = rows.next().unwrap();
            while last[i] == Rational32::ZERO {
                last = rows.next().unwrap();
            }
            for row in rows {
                let k = row[i]/last[i];
                Zip::from(&last).and(row).for_each(|&a, b| *b -= k*a);
            }
        }
    }

    fn solve(&mut self) {
        for i in 0..self.free {
            let mut x = Rational32::ZERO;
            for j in self.free..self.x.len() {
                x += self.x[j] * self.a[(i, j)];
            }
            self.x[i] = (self.a[(i, self.a.ncols()-1)] - x)/self.a[(i, i)];
        }
    }

    fn min(&mut self, j: usize) -> i32 {
        if j >= self.x.len() {
            self.solve();
            if self.x.iter().all(|&e| e.is_integer() && e >= Rational32::ZERO) {
                return self.x.iter().sum::<Rational32>().to_integer();
            } else {
                return i32::MAX;
            }
        }
        (0..=self.max_xs[j])
            .map(|x| {
                self.x[j] = x.into();
                self.min(j+1)
            })
            .min()
            .unwrap()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buttons = std::io::read_to_string(std::io::stdin())?.lines()
        .map(|line| {
            let mut machine = Machine::new(line);
            machine.gauss();
            machine.min(machine.free)
        })
        .sum::<i32>();
    println!("{}", buttons);
    Ok(())
}
