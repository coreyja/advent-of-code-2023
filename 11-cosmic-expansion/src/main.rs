#[derive(Debug, Clone)]
struct Universe(Vec<Vec<Cell>>);

impl Universe {
    fn parse(input: &str) -> Self {
        let universe = input
            .lines()
            .map(|l| l.chars().map(Cell::parse).collect())
            .collect();

        Self(universe)
    }

    fn is_row_empty(&self, row: usize) -> bool {
        self.0[row].iter().all(|c| *c == Cell::Empty)
    }

    fn is_col_empty(&self, col: usize) -> bool {
        self.0.iter().all(|r| r[col] == Cell::Empty)
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_row_empty(*i))
            .map(|(i, _)| i)
            .collect()
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_col_empty(*i))
            .map(|(i, _)| i)
            .collect()
    }

    fn expand_row(&self, row: usize) -> Self {
        let mut new_universe = self.clone();

        new_universe
            .0
            .insert(row, vec![Cell::Empty; self.0[row].len()]);

        new_universe
    }

    fn expand_col(&self, col: usize) -> Self {
        let mut new_universe = self.clone();

        for row in new_universe.0.iter_mut() {
            row.insert(col, Cell::Empty);
        }

        new_universe
    }

    fn expand(&self) -> Self {
        let mut new_universe = self.clone();

        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        for (i, row) in empty_rows.into_iter().enumerate() {
            new_universe = new_universe.expand_row(row + i);
        }

        for (i, col) in empty_cols.into_iter().enumerate() {
            new_universe = new_universe.expand_col(col + i);
        }

        new_universe
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = vec![];

        for (i, row) in self.0.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == Cell::Galaxy {
                    galaxies.push((i, j));
                }
            }
        }

        galaxies
    }

    fn galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut pairs = vec![];

        let galaxies = self.galaxies();

        for (i, galaxy) in galaxies.iter().enumerate() {
            for galaxy_match in galaxies.iter().skip(i + 1) {
                pairs.push((*galaxy, *galaxy_match));
            }
        }

        pairs
    }

    fn sum_of_distances(&self) -> usize {
        self.galaxy_pairs()
            .into_iter()
            .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Galaxy,
}

impl Cell {
    fn parse(input: char) -> Self {
        match input {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Invalid cell"),
        }
    }
}

fn part_1(input: &str) -> usize {
    // let universe = Universe::parse(input);

    // universe.expand().sum_of_distances()
    part_2(input, 2)
}

fn part_2(input: &str, expansion_factor: usize) -> usize {
    let universe = Universe::parse(input);

    let galaxies = universe.galaxies();

    struct Galaxy {
        original: (usize, usize),
        expanded: (usize, usize),
    }

    let mut galaxies = galaxies
        .into_iter()
        .map(|g| Galaxy {
            original: g,
            expanded: g,
        })
        .collect::<Vec<_>>();

    let empty_rows = universe.get_empty_rows();
    let empty_cols = universe.get_empty_cols();

    for row in empty_rows.into_iter() {
        for g in galaxies.iter_mut() {
            if g.original.0 >= row {
                g.expanded.0 += expansion_factor - 1;
            }
        }
    }

    for col in empty_cols.into_iter() {
        for g in galaxies.iter_mut() {
            if g.original.1 >= col {
                g.expanded.1 += expansion_factor - 1;
            }
        }
    }

    let pairs = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|g2| (g.expanded, g2.expanded))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    pairs
        .into_iter()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum()
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);
    dbg!(sample_part_1_ans);
    assert_eq!(sample_part_1_ans, 374);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
    assert_eq!(my_part_1_ans, 9545480);

    let sample_part_2_ans_small = part_2(sample_input, 10);
    dbg!(sample_part_2_ans_small);
    assert_eq!(sample_part_2_ans_small, 1030);

    let sample_part_2_ans_large = part_2(sample_input, 100);
    dbg!(sample_part_2_ans_large);
    assert_eq!(sample_part_2_ans_large, 8410);

    let my_part_2_ans = part_2(my_input, 1_000_000);
    dbg!(my_part_2_ans);
}
