struct History(Vec<i64>);

fn part_1(input: &str) -> i64 {
    let histories = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(History)
        .collect::<Vec<_>>();

    histories.into_iter().map(|h| h.next_item()).sum()
}

fn part_2(input: &str) -> i64 {
    let histories = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(History)
        .collect::<Vec<_>>();

    histories.into_iter().map(|h| h.previous_item()).sum()
}

impl History {
    fn next_item(&self) -> i64 {
        next_number(&self.0)
    }

    fn previous_item(&self) -> i64 {
        let mut reversed = self.0.clone();
        reversed.reverse();

        next_number(&reversed)
    }
}

fn next_number(arr: &[i64]) -> i64 {
    let differences = arr.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let last = arr.last().unwrap();

    if differences.iter().all(|&d| d == 0) {
        return *last;
    }

    let next_difference = next_number(&differences);
    *last + next_difference
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);

    let sample_part_2_ans = part_2(sample_input);
    dbg!(sample_part_2_ans);

    let my_part_2_ans = part_2(my_input);
    dbg!(my_part_2_ans);
}
