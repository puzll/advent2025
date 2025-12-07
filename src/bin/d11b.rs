use std::collections::hash_map::*;

#[derive(Clone, Copy)]
struct Paths(i64, i64, i64, i64);

impl std::ops::AddAssign for Paths {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

struct Devices {
    ids: HashMap<String, usize>,
    connections: Vec<Vec<usize>>,
    cache: Vec<Option<Paths>>,
    free_id: usize,
}

impl Devices {
    const SVR: usize = 0;
    const OUT: usize = 1;
    const DAC: usize = 2;
    const FFT: usize = 3;

    fn new() -> Self {
        let mut devices = Devices {
            ids: HashMap::new(),
            connections: Vec::new(),
            cache: Vec::new(),
            free_id: 0,
        };
        devices.id("svr");
        devices.id("out");
        devices.id("dac");
        devices.id("fft");
        devices
    }

    fn id(&mut self, name: &str) -> usize {
        match self.ids.entry(name.to_string()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let id = *entry.insert(self.free_id);
                self.free_id += 1;
                self.cache.resize(self.free_id, Option::None);
                self.connections.resize(self.free_id, Vec::new());
                id
            },
        }
    }

    fn connect(&mut self, line: &str) {
        let (name, connections) = line.split_once(':').unwrap();
        let id = self.id(name);
        self.connections[id] = connections.trim_start().split(' ')
            .map(|connection| self.id(connection))
            .collect::<Vec<_>>();
    }

    fn paths(&mut self, id: usize) -> Paths {
        if id == Devices::OUT {
            return Paths(1, 0, 0, 0);
        }
        if let Some(paths) = self.cache[id] {
            return paths;
        }
        let mut sum = Paths(0, 0, 0, 0);
        for i in 0..self.connections[id].len() {
            let child = self.connections[id][i];
            let paths @ Paths(none, dac, fft, both) = self.paths(child);
            sum += match child {
                Devices::DAC => Paths(0, none+dac, 0, fft+both),
                Devices::FFT => Paths(0, 0, none+fft, dac+both),
                _ => paths,
            }
        }
        self.cache[id] = Some(sum);
        sum
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut devices = Devices::new();
    for line in std::io::read_to_string(std::io::stdin())?.lines() {
        devices.connect(line);
    }
    println!("{}", devices.paths(Devices::SVR).3);
    Ok(())
}
