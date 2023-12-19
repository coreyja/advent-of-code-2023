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

    fn expand(&self) -> Self {
        let mut new_cells = vec![];
        for i in 0..5 {
            new_cells.append(&mut self.cells.clone());
            if i != 4 {
                new_cells.push(Cell::Unknown);
            }
        }

        let new_broken_sets = vec![self.broken_sets.clone(); 5]
            .into_iter()
            .flatten()
            .collect();

        Self {
            cells: new_cells,
            broken_sets: new_broken_sets,
        }
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
            Cell::Unknown => return sets,
        }
    }

    if current_set != 0 {
        sets.push(current_set);
    }

    sets
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PrefixMatch {
    Prefix,
    Full,
}

fn prefix_match(cell_sets: &[usize], broken_sets: &&[usize]) -> Option<PrefixMatch> {
    if cell_sets.len() > broken_sets.len() {
        return None;
    }

    for (i, c) in cell_sets.iter().enumerate() {
        if c != &broken_sets[i] {
            return None;
        }
    }

    if cell_sets.len() == broken_sets.len() {
        Some(PrefixMatch::Full)
    } else {
        Some(PrefixMatch::Prefix)
    }
}

fn count_possible(cells: &[Cell], broken_sets: &[usize]) -> usize {
    let cell_sets = to_sets(cells);

    let prefix_match = prefix_match(&cell_sets, &broken_sets);
    let Some(prefix_match) = prefix_match else {
        return 0;
    };

    let unknown_index = cells.iter().position(|c| *c == Cell::Unknown);
    let Some(unknown_index) = unknown_index else {
        if prefix_match == PrefixMatch::Full {
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

fn part_2(input: &str) -> usize {
    // This answer is not optimized and takes a long time to run.
    //
    // I think I want to change the `count_possible` impl to work on groups of things
    // instead of individual characters.
    //
    // Get groupings of each cell, and compare with the 'broken sets' to look for pre/post fixes that 'match'.
    // By match we mean a group with size that matches the broken set
    // We can then 'strip' those matches out and work on the smaller inner groups.
    //
    // At some point we will need to break up the groups into smaller groups
    // We can do that by making each unknown a working cell, and using that to create smaller groups.
    // We can sum the result for making each unknown a working cell
    let rows = input.lines().map(Row::parse).collect::<Vec<_>>();
    let expanded_rows = rows.iter().map(|r| r.expand()).collect::<Vec<_>>();

    expanded_rows.into_iter().map(|r| r.count_possible()).sum()
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
