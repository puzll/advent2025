use std::collections::hash_map::*;

struct Devices {
    ids: HashMap<String, usize>,
    connections: Vec<Vec<usize>>,
    free_id: usize,
}

impl Devices {
    const YOU: usize = 0;
    const OUT: usize = 1;

    fn new() -> Self {
        let mut devices = Devices {
            ids: HashMap::new(),
            connections: Vec::new(),
            free_id: 0,
        };
        devices.id("you");
        devices.id("out");
        devices
    }

    fn id(&mut self, name: &str) -> usize {
        match self.ids.entry(name.to_string()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let id = *entry.insert(self.free_id);
                self.free_id += 1;
                id
            },
        }
    }

    fn connect(&mut self, line: &str) {
        let (name, connections) = line.split_once(':').unwrap();
        let id = self.id(name);
        self.connections.resize(self.connections.len().max(id+1), Vec::new());
        self.connections[id] = connections.trim_start().split(' ')
            .map(|connection| self.id(connection))
            .collect::<Vec<_>>();
    }

    fn paths(&self, id: usize) -> i32 {
        if id == Devices::OUT {
            return 1;
        }
        self.connections[id].iter()
            .map(|&child| self.paths(child))
            .sum()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut devices = Devices::new();
    for line in std::io::read_to_string(std::io::stdin())?.lines() {
        devices.connect(line);
    }
    println!("{}", devices.paths(Devices::YOU));
    Ok(())
}
