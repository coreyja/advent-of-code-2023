use std::ops::{Range, RangeInclusive};

struct Input {
    races: Vec<Race>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let times = lines[0];
        let distances = lines[1];

        let times = times.strip_prefix("Time:").unwrap().trim();
        let distances = distances.strip_prefix("Distance:").unwrap().trim();

        let times = times.split_whitespace().map(|s| s.parse::<u64>().unwrap());
        let distances = distances
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let races = times
            .zip(distances)
            .map(|(time, record_distance)| Race {
                time,
                record_distance,
            })
            .collect::<Vec<_>>();

        Input { races }
    }
}

struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn possible_win_range(&self) -> RangeInclusive<u64> {
        let mut min = None;
        let mut max = None;
        for i in 1..=self.time {
            let velocity = i;
            let time_left = self.time - i;

            let distance = velocity * time_left;

            if distance > self.record_distance {
                if min.is_none() {
                    min = Some(i);
                }
                max = Some(i);
            }
        }

        min.unwrap()..=max.unwrap()
    }
}

fn part_1(input: &str) -> u64 {
    let input = Input::parse(input);
    input
        .races
        .iter()
        .map(|r| r.possible_win_range().count())
        .reduce(|a, b| a * b)
        .unwrap() as u64
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
}
