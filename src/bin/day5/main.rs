static INPUT_FILE: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Mapping {
    origin: u64,
    destination: u64,
    range: u64,
}

impl Mapping {
    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        Mapping {
            destination: parts[0].parse().unwrap(),
            origin: parts[1].parse().unwrap(),
            range: parts[2].parse().unwrap(),
        }
    }
}

fn mappings<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<Mapping>> {
    let mut mappings: Vec<Vec<Mapping>> = vec![];

    for line in input {
        if line.is_empty() {
            mappings.push(vec![]);
        } else {
            mappings.last_mut().unwrap().push(Mapping::new(line));
        }
    }

    mappings
}

fn traverse_mappings(seed: u64, mappings: &[Vec<Mapping>]) -> u64 {
    mappings.iter().fold(seed, |acc, m| {
        m.iter()
            .find(|&map| acc >= map.origin && acc < map.origin + map.range)
            .map_or(acc, |map| map.destination + acc - map.origin)
    })
}

fn main() {
    let mut input = INPUT_FILE.lines();
    let seeds: Vec<u64> = input
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = mappings(input);
    println!(
        "part 1: {}",
        seeds
            .iter()
            .map(|seed| traverse_mappings(*seed, &maps))
            .min()
            .unwrap()
    );
}
