pub(crate) mod part_1;
pub(crate) mod part_2;

fn part_1(sample_input: &str) -> usize {
    let mut input = part_1::Input::parse(sample_input);
    input.hands.sort();

    input
        .hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let multiplier = i + 1;

            h.bid * multiplier
        })
        .sum()
}

fn part_2(sample_input: &str) -> usize {
    let mut input = part_2::Input::parse(sample_input);
    input.hands.sort();

    input
        .hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let multiplier = i + 1;

            h.bid * multiplier
        })
        .sum()
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
