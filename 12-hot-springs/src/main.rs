#[derive(Debug)]
struct Row {
    cells: Vec<Cell>,
    broken_sets: Vec<usize>,
}

impl Row {
    fn parse(input: &str) -> Self {
        let mut split = input.split(' ');
        let cells = split.next().unwrap();
        let broken_sets = split.next().unwrap();

        let cells = cells.chars().map(Cell::parse).collect::<Vec<_>>();

        let broken_sets = broken_sets.split(',').map(|s| s.parse().unwrap()).collect();

        Self { cells, broken_sets }
    }

    fn count_possible(&self) -> usize {
        count_possible(&self.cells, &self.broken_sets)
    }
}

fn to_sets(cells: &[Cell]) -> Vec<usize> {
    let mut sets = Vec::new();

    let mut current_set = 0;
    for cell in cells {
        match cell {
            Cell::Working => {
                if current_set != 0 {
                    sets.push(current_set);
                }
                current_set = 0;
            }
            Cell::Broken => {
                current_set += 1;
            }
            Cell::Unknown => panic!("Unknown cell when counting sets"),
        }
    }

    if current_set != 0 {
        sets.push(current_set);
    }

    sets
}

fn count_possible(cells: &[Cell], broken_sets: &[usize]) -> usize {
    let unknown_index = cells.iter().position(|c| *c == Cell::Unknown);
    let Some(unknown_index) = unknown_index else {
        let cell_sets = to_sets(cells);

        if cell_sets == broken_sets {
            return 1;
        } else {
            return 0;
        }
    };

    let mut working_cells: Vec<_> = cells.to_vec();
    let mut broken_cells: Vec<_> = cells.to_vec();

    working_cells[unknown_index] = Cell::Working;
    broken_cells[unknown_index] = Cell::Broken;

    let working_count = count_possible(&working_cells, broken_sets);
    let broken_count = count_possible(&broken_cells, broken_sets);

    working_count + broken_count
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Working,
    Broken,
    Unknown,
}

impl Cell {
    fn parse(input: char) -> Self {
        match input {
            '.' => Self::Working,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => panic!("Invalid cell"),
        }
    }
}

fn part_1(input: &str) -> usize {
    let rows = input.lines().map(Row::parse).collect::<Vec<_>>();

    rows.into_iter().map(|r| r.count_possible()).sum()
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
}
