use miette::Result;

#[derive(Debug, PartialEq)]
struct GridNumber {
    num: u32,
    y: usize,
    x_start: usize,
    x_end: usize,
}
impl GridNumber {
    fn is_part_number(&self, array: &[Vec<char>]) -> bool {
        'outer: for ny in [self.y.checked_sub(1), Some(self.y), Some(self.y + 1)]
            .iter()
            .flatten()
        {
            if *ny >= array.len() {
                continue;
            }

            let mut nx = self.x_start.saturating_sub(1);
            while nx <= self.x_end + 1 {
                if nx >= array[*ny].len() {
                    continue 'outer;
                }

                if array[*ny][nx].is_ascii_punctuation() && array[*ny][nx] != '.' {
                    return true;
                }

                nx += 1;
            }
        }

        false
    }
}

fn part_1(input: &str) -> Result<u32> {
    let array = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid_numbers = vec![];

    let mut i = 0;
    while i < array.len() {
        let mut j = 0;
        let mut digit_started_at: Option<usize> = None;

        while j < array[i].len() {
            let c = array[i][j];

            if c.is_ascii_digit() {
                if digit_started_at.is_none() {
                    digit_started_at = Some(j);
                }
            } else if let Some(start) = digit_started_at {
                let number = array[i][start..j]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                let grid_number = GridNumber {
                    num: number,
                    y: i,
                    x_start: start,
                    x_end: j - 1,
                };
                grid_numbers.push(grid_number);

                digit_started_at = None;
            };

            j += 1;
        }
        if let Some(start) = digit_started_at {
            let number = array[i][start..j]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let grid_number = GridNumber {
                num: number,
                y: i,
                x_start: start,
                x_end: j - 1,
            };
            grid_numbers.push(grid_number);
        }

        i += 1;
    }

    Ok(grid_numbers
        .iter()
        .filter(|gn| gn.is_part_number(&array))
        .map(|gn| gn.num)
        .sum::<u32>())
}

fn main() -> Result<()> {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input)?;

    dbg!(sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input)?;

    dbg!(my_part_1_ans);

    Ok(())
}
