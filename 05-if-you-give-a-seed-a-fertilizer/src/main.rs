fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut sections = input.split("\n\n");

        let seeds = sections.next().unwrap();
        let seeds = seeds.strip_prefix("seeds: ").unwrap();
        let seeds = seeds
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        let maps = sections.map(Map::parse).collect::<Vec<_>>();

        Self { seeds, maps }
    }

    fn mapped_value(&self, mut seed: u64) -> u64 {
        for map in &self.maps {
            let entry: Option<u64> = map.entries.iter().find_map(|e| e.translate(seed));
            seed = entry.unwrap_or(seed);
        }

        seed
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    entries: Vec<MapEntry>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let name_line = lines.next().unwrap();
        let name = name_line.split(' ').next().unwrap().to_string();

        let entries = lines.map(MapEntry::parse).collect::<Vec<_>>();

        Self { name, entries }
    }
}

#[derive(Debug)]
struct MapEntry {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl MapEntry {
    fn parse(l: &str) -> Self {
        let nums = l
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u64>>();

        Self {
            dest_range_start: nums[0],
            source_range_start: nums[1],
            range_length: nums[2],
        }
    }

    fn translate(&self, seed: u64) -> Option<u64> {
        if seed >= self.source_range_start && seed < self.source_range_start + self.range_length {
            let offset = seed - self.source_range_start;
            Some(self.dest_range_start + offset)
        } else {
            None
        }
    }
}

fn part_1(sample_input: &str) -> u64 {
    let input = Input::parse(sample_input);

    input
        .seeds
        .iter()
        .map(|seed| input.mapped_value(*seed))
        .min()
        .unwrap()
}
