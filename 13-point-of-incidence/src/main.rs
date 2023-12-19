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
    fn is_horizontal_reflection(&self, index: usize) -> bool {
        if index == 0 {
            return false;
        }

        let max_distance = index.min(self.cells.len() - index);

        for distance in 1..=max_distance {
            let top = index - distance;
            let bottom = index + distance - 1;

            for x in 0..self.cells[top].len() {
                if self.cells[top][x] != self.cells[bottom][x] {
                    return false;
                }
            }
        }

        true
    }

    fn is_vertical_reflection(&self, index: usize) -> bool {
        if index == 0 {
            return false;
        }

        let max_distance = index.min(self.cells[0].len() - index);

        for distance in 1..=max_distance {
            let left = index - distance;
            let right = index + distance - 1;

            for y in 0..self.cells.len() {
                if self.cells[y][left] != self.cells[y][right] {
                    return false;
                }
            }
        }

        true
    }

    fn horizontal_reflection(&self) -> Option<usize> {
        (0..self.cells.len()).find(|&i| self.is_horizontal_reflection(i))
    }

    fn vertical_reflection(&self) -> Option<usize> {
        (0..self.cells[0].len()).find(|&i| self.is_vertical_reflection(i))
    }

    fn value(&self) -> usize {
        let hor = self.horizontal_reflection();
        if let Some(hor) = hor {
            return hor * 100;
        }

        let ver = self.vertical_reflection();
        if let Some(ver) = ver {
            return ver;
        }

        let s = self
            .cells
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", s);
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

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(&sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(&my_part_1_ans);
}
