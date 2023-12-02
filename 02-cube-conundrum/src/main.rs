#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cube_draws: Vec<CubeDraw>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let split = input.split(':').collect::<Vec<_>>();
        let game_id_section = split[0];
        let cube_draws_section = split[1];

        let game_id = game_id_section.split_whitespace().collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();

        let cube_draws: Vec<_> = cube_draws_section.split(';').map(CubeDraw::parse).collect();

        Game {
            id: game_id,
            cube_draws,
        }
    }

    fn valid_for(&self, validate_against: &CubeDraw) -> bool {
        self.cube_draws
            .iter()
            .all(|cd| cd.valid_for(validate_against))
    }
}

#[derive(Debug, PartialEq)]
struct CubeDraw {
    red_count: u32,
    blue_count: u32,
    green_count: u32,
}

impl CubeDraw {
    fn parse(input: &str) -> Self {
        // Format of each colored_draw is vec!["8","green"]
        let colored_draw = input
            .split(',')
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        fn count_for_color(colored_draw: &[Vec<&str>], color: &str) -> u32 {
            colored_draw
                .iter()
                .find(|cd| cd[1] == color)
                .map(|cd| cd[0].parse::<u32>().unwrap())
                .unwrap_or(0)
        }

        let red_count = count_for_color(&colored_draw, "red");
        let blue_count = count_for_color(&colored_draw, "blue");
        let green_count = count_for_color(&colored_draw, "green");

        CubeDraw {
            red_count,
            blue_count,
            green_count,
        }
    }

    fn valid_for(&self, validate_against: &CubeDraw) -> bool {
        self.red_count <= validate_against.red_count
            && self.blue_count <= validate_against.blue_count
            && self.green_count <= validate_against.green_count
    }
}

fn part_1(input: &str) -> u32 {
    let validate_against = CubeDraw {
        red_count: 12,
        blue_count: 14,
        green_count: 13,
    };

    let games = input.lines().map(Game::parse).collect::<Vec<_>>();

    let valid_games = games
        .iter()
        .filter(|g| g.valid_for(&validate_against))
        .collect::<Vec<_>>();

    valid_games.iter().map(|g| g.id).sum()
}

fn main() {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);

    dbg!(sample_part_1_ans, my_part_1_ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cube_draw() {
        let input = "8 green, 6 blue";
        let cd = CubeDraw::parse(input);

        assert_eq!(
            cd,
            CubeDraw {
                red_count: 0,
                blue_count: 6,
                green_count: 8
            }
        )
    }
}
