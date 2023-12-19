struct Puzzle {
    cells: Vec<Vec<char>>,
}

impl Puzzle {
    fn parse(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { cells }
    }

    // index is a line in the horizontal direction
    // between two y-indexes
    fn horizontal_reflection_count_differences(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }

        let max_distance = index.min(self.cells.len() - index);
        let mut differences = 0;

        for distance in 1..=max_distance {
            let top = index - distance;
            let bottom = index + distance - 1;

            for x in 0..self.cells[top].len() {
                if self.cells[top][x] != self.cells[bottom][x] {
                    differences += 1;
                }
            }
        }

        Some(differences)
    }

    fn vertical_reflection_count_differences(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }

        let max_distance = index.min(self.cells[0].len() - index);
        let mut differences = 0;

        for distance in 1..=max_distance {
            let left = index - distance;
            let right = index + distance - 1;

            for y in 0..self.cells.len() {
                if self.cells[y][left] != self.cells[y][right] {
                    differences += 1;
                }
            }
        }

        Some(differences)
    }

    fn horizontal_reflection(&self, num_differences: usize) -> Option<usize> {
        (0..self.cells.len())
            .find(|&i| self.horizontal_reflection_count_differences(i) == Some(num_differences))
    }

    fn vertical_reflection(&self, num_differences: usize) -> Option<usize> {
        (0..self.cells[0].len())
            .find(|&i| self.vertical_reflection_count_differences(i) == Some(num_differences))
    }

    fn value(&self) -> usize {
        let hor = self.horizontal_reflection(0);
        if let Some(hor) = hor {
            return hor * 100;
        }

        let ver = self.vertical_reflection(0);
        if let Some(ver) = ver {
            return ver;
        }

        panic!("No reflection found");
    }

    fn part_2_value(&self) -> usize {
        let hor = self.horizontal_reflection(1);
        if let Some(hor) = hor {
            return hor * 100;
        }

        let ver = self.vertical_reflection(1);
        if let Some(ver) = ver {
            return ver;
        }

        panic!("No reflection found");
    }
}

fn part_1(sample_input: &str) -> usize {
    let puzzles = sample_input
        .split("\n\n")
        .map(Puzzle::parse)
        .collect::<Vec<_>>();

    puzzles.iter().map(|p| p.value()).sum()
}

fn part_2(sample_input: &str) -> usize {
    let puzzles = sample_input
        .split("\n\n")
        .map(Puzzle::parse)
        .collect::<Vec<_>>();

    puzzles.iter().map(|p| p.part_2_value()).sum()
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(&sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(&my_part_1_ans);

    let sample_part_2_ans = part_2(sample_input);
    dbg!(&sample_part_2_ans);

    let my_part_2_ans = part_2(my_input);
    dbg!(&my_part_2_ans);
}
