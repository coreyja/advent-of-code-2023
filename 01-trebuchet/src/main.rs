fn process_line_part_1(line: &str) -> u32 {
    let number_chars = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();

    let combined = format!(
        "{}{}",
        number_chars[0],
        number_chars[number_chars.len() - 1]
    );

    println!("{}", combined);

    combined.parse::<u32>().unwrap()
}

fn part_1(input: &str) -> u32 {
    input.lines().map(process_line_part_1).sum()
}

fn part_2(input: &str) -> u32 {
    input.lines().map(process_line_part_2).sum()
}

#[derive(Debug, Copy, Clone)]
struct Indexes {
    pos: usize,
    num: u32,
}

fn process_line_part_2(input: &str) -> u32 {
    let searches = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ];

    let mut found = searches
        .iter()
        .filter_map(|(word, num)| input.find(word).map(|pos| Indexes { pos, num: *num }))
        .collect::<Vec<_>>();
    let mut last_found = searches
        .iter()
        .filter_map(|(word, num)| input.rfind(word).map(|pos| Indexes { pos, num: *num }))
        .collect::<Vec<_>>();

    found.append(&mut last_found);

    found.sort_by_key(|x| x.pos);

    let first = found.first().unwrap();
    let last = found.last().unwrap();

    let ans = (first.num * 10) + last.num;

    // dbg!(input, ans);

    ans
}

fn process_line_part_2_opus(line: &str) -> u32 {
    let replacements = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut actual_replacements = replacements
        .iter()
        .filter_map(|(word, num)| {
            let pos = line.find(word);
            pos.map(|pos| (pos, word, num))
        })
        .collect::<Vec<_>>();
    actual_replacements.sort_by_key(|x| x.0);

    let mut line = line.to_string();

    if let Some(first_replacement) = actual_replacements.first() {
        line = line.replace(first_replacement.1, &first_replacement.2.to_string());
    };
    if let Some(last_replacement) = actual_replacements.last() {
        line = line.replace(last_replacement.1, &last_replacement.2.to_string());
    }

    part_1(&line)
}

fn main() {
    let sample = include_str!("sample.input").trim();
    let sample_answer_part_1 = part_1(sample);

    dbg!(sample_answer_part_1);

    let input = include_str!("my.input").trim();
    let my_answer_part_1 = part_1(input);
    dbg!(my_answer_part_1);

    let sample_answer_part_2 = part_2(include_str!("sample2.input").trim());
    let my_answer_part_2 = part_2(input);

    dbg!(sample_answer_part_2, my_answer_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_case() {
        let line = "mtthreeclxhfivep8threelh";
        let ans = process_line_part_2(line);
        assert_eq!(ans, 33);
    }
}
