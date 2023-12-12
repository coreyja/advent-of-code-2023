#![allow(dead_code)]

use std::ops::Index;

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
            Cell::Pipe(pipe) => pipe.connection_positions(position, self),
        }
    }

    fn neighbor_pipes(&self, position: Position) -> Vec<Position> {
        self.neighbors(position)
            .into_iter()
            .filter(|(_, pos)| match self.grid[pos.y][pos.x] {
                Cell::Pipe(_) => true,
                _ => false,
            })
            .map(|(_, pos)| pos)
            .collect()
    }

    fn neighbors(&self, position: Position) -> Vec<(Direction, Position)> {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        directions
            .into_iter()
            .filter_map(|d| self.in_direction(position, d).map(|p| (d, p)))
            .collect::<Vec<_>>()
    }

    fn is_inside(&self, main_loop: &[Position], pos: Position) -> bool {
        if main_loop.contains(&pos) {
            return false;
        }

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let odd_directions = directions
            .iter()
            .filter(|d| self.count_pipes_in_direction(main_loop, pos, **d) % 2 == 1)
            .collect::<Vec<_>>();

        let vertically_inside =
            odd_directions.contains(&&Direction::Up) && odd_directions.contains(&&Direction::Down);

        let horizontally_inside = odd_directions.contains(&&Direction::Left)
            && odd_directions.contains(&&Direction::Right);

        // let odd_directions_that_arent_edges = odd_directions
        //     .iter()
        //     .filter(|d| self.in_direction(pos, d.opposite()).is_some())
        //     .collect::<Vec<_>>();

        vertically_inside || horizontally_inside
        // !odd_directions_that_arent_edges.is_empty()
    }

    fn count_pipes_in_direction(
        &self,
        main_loop: &[Position],
        pos: Position,
        d: Direction,
    ) -> usize {
        let mut pos = Some(pos);
        let mut count = 0;

        while let Some(p) = pos {
            pos = self.in_direction(p, d);

            if main_loop.contains(&p) {
                count += 1
            }
        }

        count
    }

    fn in_direction(&self, pos: Position, d: Direction) -> Option<Position> {
        match d {
            Direction::Up if pos.y >= 1 => Some(Position {
                x: pos.x,
                y: pos.y - 1,
            }),
            Direction::Down if pos.y < self.grid.len() - 1 => Some(Position {
                x: pos.x,
                y: pos.y + 1,
            }),
            Direction::Left if pos.x >= 1 => Some(Position {
                x: pos.x - 1,
                y: pos.y,
            }),
            Direction::Right if pos.x < self.grid[0].len() - 1 => Some(Position {
                x: pos.x + 1,
                y: pos.y,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn direction_to(&self, other: Position) -> Direction {
        if (self.x == other.x) && (self.y + 1 == other.y) {
            return Direction::Down;
        }
        if (self.x == other.x) && (self.y == other.y + 1) {
            return Direction::Up;
        }
        if (self.x + 1 == other.x) && (self.y == other.y) {
            return Direction::Right;
        }
        if (self.x == other.x + 1) && (self.y == other.y) {
            return Direction::Left;
        }

        panic!("Not adjacent");
    }
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
    fn connection_positions(&self, position: Position, maze: &Maze) -> Vec<Position> {
        self.connections
            .iter()
            .map(|d| maze.in_direction(position, *d).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn part_1(input: &str) -> usize {
    let maze = Maze::parse(input);
    let main_loop = maze.get_main_loop();

    main_loop.len() / 2
}

fn part_2(input: &str) -> usize {
    let maze = Maze::parse(input);
    let main_loop = maze.get_main_loop();

    let mut visited = vec![];
    let mut inside = vec![];
    struct ToVisit {
        position: Position,
        inside: bool,
    }
    let mut to_visit = vec![ToVisit {
        position: Position { x: 0, y: 0 },
        inside: false,
    }];

    while let Some(to) = to_visit.pop() {
        let pos = to.position;

        if to.inside && !main_loop.contains(&pos) {
            inside.push(pos);
        }

        if visited.contains(&pos) {
            continue;
        }
        visited.push(pos);

        let neighbors = maze.neighbors(pos).into_iter().map(|(dir, p)| {
            let opposite = dir.opposite();

            let mut inside = to.inside;

            // Enterting a pipe
            if main_loop.contains(&p) {
                let pipe = maze.grid[p.y][p.x].clone();
                let connections = match pipe {
                    Cell::Pipe(pipe) => pipe.connections,
                    Cell::Start => maze
                        .connections(p)
                        .into_iter()
                        .map(|new| p.direction_to(new))
                        .collect(),
                    _ => {
                        panic!("Not a pipe");
                    }
                };

                if connections.contains(&opposite) ^ connections.contains(&dir) {
                    inside = !inside;
                }
            }

            // // Exiting a pipe
            // THIS IS WRONG, WE DON'T TO COUNT EXISTS AS WELL AS ENTRANCES
            // if main_loop.contains(&pos) {
            //     let connections = match &maze.grid[pos.y][pos.x] {
            //         Cell::Pipe(pipe) => pipe.connections.clone(),
            //         Cell::Start => maze
            //             .connections(pos)
            //             .into_iter()
            //             .map(|new| pos.direction_to(new))
            //             .collect(),
            //         _ => {
            //             panic!("Not a pipe");
            //         }
            //     };
            //     if connections.contains(&opposite) ^ connections.contains(&dir) {
            //         inside = !inside;
            //     }
            // }
            if p == (Position { x: 2, y: 2 }) {
                dbg!(inside);
            };

            ToVisit {
                position: p,
                inside,
            }
        });

        to_visit.extend(neighbors);
    }

    dbg!(&inside);
    inside.len()
}

fn main() {
    let simple_sample_input = include_str!("simple_sample.input");
    let simple_sample_part_1_ans = part_1(simple_sample_input);
    dbg!(simple_sample_part_1_ans);
    assert_eq!(simple_sample_part_1_ans, 4);

    let complex_sample_input = include_str!("complex_sample.input");
    let complex_sample_part_1_ans = part_1(complex_sample_input);
    dbg!(complex_sample_part_1_ans);
    assert_eq!(complex_sample_part_1_ans, 8);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);
    assert_eq!(my_part_1_ans, 6725);

    let simple_sample_part_2_ans = part_2(simple_sample_input);
    dbg!(simple_sample_part_2_ans);
    assert_eq!(simple_sample_part_2_ans, 1);

    let inside_sample_1 = include_str!("inside_sample_1.input");
    let inside_sample_1_part_2_ans = part_2(inside_sample_1);
    dbg!(inside_sample_1_part_2_ans);
    assert_eq!(inside_sample_1_part_2_ans, 4);

    let inside_sample_2 = include_str!("inside_sample_2.input");
    let inside_sample_2_part_2_ans = part_2(inside_sample_2);
    dbg!(inside_sample_2_part_2_ans);
    assert_eq!(inside_sample_2_part_2_ans, 4);

    let large_sample = include_str!("larger_sample.input");
    let large_sample_part_2_ans = part_2(large_sample);
    dbg!(large_sample_part_2_ans);
    assert_eq!(large_sample_part_2_ans, 8);

    // let final_sample_input = include_str!("final_sample.input");
    // let final_sample_part_2_ans = part_2(final_sample_input);
    // dbg!(final_sample_part_2_ans);

    // let my_part_2_ans = part_2(my_input);
    // dbg!(my_part_2_ans);
}
