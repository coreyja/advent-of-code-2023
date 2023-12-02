use miette::{IntoDiagnostic, Result};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cube_draws: Vec<CubeDraw>,
}

impl Game {
    fn parse(input: &str) -> Result<Self> {
        let split = input.split(':').collect::<Vec<_>>();
        let game_id_section = split[0];
        let cube_draws_section = split[1];

        let game_id = game_id_section.split_whitespace().collect::<Vec<_>>()[1]
            .parse::<u32>()
            .into_diagnostic()?;

        let cube_draws: Result<Vec<_>> =
            cube_draws_section.split(';').map(CubeDraw::parse).collect();

        let cube_draws = cube_draws?;

        Ok(Game {
            id: game_id,
            cube_draws,
        })
    }

    fn valid_for(&self, validate_against: &CubeDraw) -> bool {
        self.cube_draws
            .iter()
            .all(|cd| cd.valid_for(validate_against))
    }

    fn minumum_power(&self) -> u32 {
        let mut minimum_cube_count = CubeDraw {
            red_count: 0,
            blue_count: 0,
            green_count: 0,
        };

        for cd in &self.cube_draws {
            if cd.red_count > minimum_cube_count.red_count {
                minimum_cube_count.red_count = cd.red_count;
            }

            if cd.blue_count > minimum_cube_count.blue_count {
                minimum_cube_count.blue_count = cd.blue_count;
            }

            if cd.green_count > minimum_cube_count.green_count {
                minimum_cube_count.green_count = cd.green_count;
            }
        }

        minimum_cube_count.red_count
            * minimum_cube_count.blue_count
            * minimum_cube_count.green_count
    }
}

#[derive(Debug, PartialEq)]
struct CubeDraw {
    red_count: u32,
    blue_count: u32,
    green_count: u32,
}

impl CubeDraw {
    fn parse(input: &str) -> Result<Self> {
        // Format of each colored_draw is vec!["8","green"]
        let colored_draw = input
            .split(',')
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .map(|v| (v[0], v[1]))
            .collect::<Vec<_>>();

        fn count_for_color(colored_draw: &[(&str, &str)], color: &str) -> Result<u32> {
            colored_draw
                .iter()
                .find(|cd| cd.1 == color)
                .map(|cd| cd.0.parse::<u32>().into_diagnostic())
                .unwrap_or(Ok(0))
        }

        let red_count = count_for_color(&colored_draw, "red")?;
        let blue_count = count_for_color(&colored_draw, "blue")?;
        let green_count = count_for_color(&colored_draw, "green")?;

        Ok(CubeDraw {
            red_count,
            blue_count,
            green_count,
        })
    }

    fn valid_for(&self, validate_against: &CubeDraw) -> bool {
        self.red_count <= validate_against.red_count
            && self.blue_count <= validate_against.blue_count
            && self.green_count <= validate_against.green_count
    }
}

fn part_1(input: &str) -> Result<u32> {
    let validate_against = CubeDraw {
        red_count: 12,
        blue_count: 14,
        green_count: 13,
    };

    let games = input.lines().map(Game::parse).collect::<Result<Vec<_>>>()?;

    let valid_games = games
        .iter()
        .filter(|g| g.valid_for(&validate_against))
        .collect::<Vec<_>>();

    Ok(valid_games.iter().map(|g| g.id).sum())
}

fn part_2(input: &str) -> Result<u32> {
    let games = input.lines().map(Game::parse).collect::<Result<Vec<_>>>()?;

    Ok(games.iter().map(|g| g.minumum_power()).sum())
}

fn main() -> Result<()> {
    let sample_input = include_str!("sample.input");
    let sample_part_1_ans = part_1(sample_input)?;

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input)?;

    dbg!(sample_part_1_ans, my_part_1_ans);

    let sample_part_2_ans = part_2(sample_input)?;
    dbg!(sample_part_2_ans);

    let my_part_2_ans = part_2(my_input)?;
    dbg!(my_part_2_ans);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cube_draw() -> Result<()> {
        let input = "8 green, 6 blue";
        let cd = CubeDraw::parse(input)?;

        assert_eq!(
            cd,
            CubeDraw {
                red_count: 0,
                blue_count: 6,
                green_count: 8
            }
        );

        Ok(())
    }
}
