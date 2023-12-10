#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.chars().map(Cell::parse).collect())
            .collect();

        Self { grid }
    }

    fn start(&self) -> Position {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Start = cell {
                    return Position { x, y };
                }
            }
        }

        panic!("No start found");
    }

    fn get_main_loop(&self) -> Vec<Position> {
        let start = self.start();
        let mut main_loop = vec![];

        let mut position = start;
        let mut previous_position = self.connections(start)[0];

        loop {
            main_loop.push(position);

            let connections = self.connections(position);
            let next_connection = connections
                .iter()
                .filter(|pos| **pos != previous_position)
                .cloned()
                .next()
                .unwrap();

            previous_position = position;
            position = next_connection;

            if next_connection == start {
                break;
            }
        }

        main_loop
    }

    fn connections(&self, position: Position) -> Vec<Position> {
        let cell = self.grid[position.y][position.x].clone();

        match cell {
            Cell::Start => {
                let pipes = self.neighbor_pipes(position);
                let start_connections = pipes
                    .iter()
                    .filter(|pos| self.connections(**pos).contains(&position))
                    .cloned()
                    .collect::<Vec<_>>();

                assert_eq!(start_connections.len(), 2);

                start_connections
            }
            Cell::Empty => panic!("Empty cell has no connections"),
            Cell::Pipe(pipe) => pipe.connection_positions(position),
        }
    }

    fn neighbor_pipes(&self, position: Position) -> Vec<Position> {
        let mut pipes = vec![];

        for y_diff in -1..=1 {
            for x_diff in -1..=1 {
                if y_diff == 0 && x_diff == 0 {
                    continue;
                }

                let x = position.x as isize + x_diff;
                let y = position.y as isize + y_diff;

                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if y >= self.grid.len() || x >= self.grid[y].len() {
                    continue;
                }

                if let Cell::Empty = self.grid[y][x] {
                    continue;
                }

                pipes.push(Position { x, y });
            }
        }

        pipes
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Cell {
    Start,
    Empty,
    Pipe(Pipe),
}

impl Cell {
    fn parse(input: char) -> Self {
        match input {
            'S' => Cell::Start,
            '.' => Cell::Empty,
            '|' => Cell::Pipe(Pipe {
                connections: vec![Direction::Up, Direction::Down],
            }),
            '-' => Cell::Pipe(Pipe {
                connections: vec![Direction::Left, Direction::Right],
            }),
            '7' => Cell::Pipe(Pipe {
                connections: vec![Direction::Left, Direction::Down],
            }),
            'F' => Cell::Pipe(Pipe {
                connections: vec![Direction::Right, Direction::Down],
            }),
            'L' => Cell::Pipe(Pipe {
                connections: vec![Direction::Right, Direction::Up],
            }),
            'J' => Cell::Pipe(Pipe {
                connections: vec![Direction::Left, Direction::Up],
            }),
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Pipe {
    connections: Vec<Direction>,
}
impl Pipe {
    fn connection_positions(&self, position: Position) -> Vec<Position> {
        self.connections
            .iter()
            .map(|d| match d {
                Direction::Up => Position {
                    x: position.x,
                    y: position.y - 1,
                },
                Direction::Down => Position {
                    x: position.x,
                    y: position.y + 1,
                },
                Direction::Left => Position {
                    x: position.x - 1,
                    y: position.y,
                },
                Direction::Right => Position {
                    x: position.x + 1,
                    y: position.y,
                },
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_1(input: &str) -> usize {
    let maze = Maze::parse(input);
    let main_loop = maze.get_main_loop();

    main_loop.len() / 2
}

fn main() {
    let simple_sample_input = include_str!("simple_sample.input");
    let simple_sample_part_1_ans = part_1(simple_sample_input);
    dbg!(simple_sample_part_1_ans);

    let complex_sample_input = include_str!("complex_sample.input");
    let complex_sample_part_1_ans = part_1(complex_sample_input);
    dbg!(complex_sample_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
}
