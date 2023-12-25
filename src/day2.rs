use std::io;
use std::io::BufRead;
use std::io::Stdin;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    cubes: Vec<Cube>,
}

struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(cube: &str) -> Result<Self> {
        let mut split = cube.split(' ');

        let quantity = split
            .next()
            .map(usize::from_str)
            .ok_or_else(|| anyhow!("Missing quantity of cube(s)!"))??;

        let cube = split
            .next()
            .map(|color| match color {
                "red" => Ok(Self::Red(quantity)),
                "green" => Ok(Self::Green(quantity)),
                "blue" => Ok(Self::Blue(quantity)),
                _ => Err(anyhow!("Unknown cube color!")),
            })
            .ok_or_else(|| anyhow!("Missing cube color!"))??;

        Ok(cube)
    }
}

impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(cube_set: &str) -> Result<Self> {
        cube_set
            .split(", ")
            .map(Cube::from_str)
            .collect::<Result<Vec<_>>>()
            .map(|cubes| Self { cubes })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(game: &str) -> Result<Self> {
        let mut game_split = game.split(": ");

        let id = game_split
            .next()
            .and_then(|game| game.split(' ').last())
            .map(usize::from_str)
            .ok_or_else(|| anyhow!("Missing game ID!"))??;

        let cube_sets = game_split
            .next()
            .map(|game| game.split("; "))
            .map(|cube_sets| cube_sets.map(CubeSet::from_str))
            .ok_or_else(|| anyhow!("Missing game revealed cubes!"))?
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { id, cube_sets })
    }
}

impl Cube {
    fn is_possible(&self, (red, green, blue): (Cube, Cube, Cube)) -> bool {
        if let (Self::Red(this), Self::Red(other)) = (*self, red) {
            this <= other
        } else if let (Self::Green(this), Self::Green(other)) = (*self, green) {
            this <= other
        } else if let (Self::Blue(this), Self::Blue(other)) = (*self, blue) {
            this <= other
        } else {
            false
        }
    }
}

impl CubeSet {
    fn is_possible(&self, cubes: (Cube, Cube, Cube)) -> bool {
        self.cubes.iter().all(|cube| cube.is_possible(cubes))
    }
}

impl Game {
    fn from_stdin(stdin: Stdin) -> Result<Vec<Self>> {
        stdin
            .lock()
            .lines()
            .take_while(|line| {
                line.as_deref()
                    .map(|line| !line.is_empty())
                    .unwrap_or_default()
            })
            .map(|line| line.map_err(|err| anyhow!(err)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|game| Game::from_str(game.as_str()))
            .collect::<Result<_>>()
    }

    fn is_possible(&self, cubes: (Cube, Cube, Cube)) -> bool {
        self.cube_sets
            .iter()
            .all(|cube_set| cube_set.is_possible(cubes))
    }
}

fn part_one(games: &[Game]) -> usize {
    const RED: Cube = Cube::Red(12);
    const GREEN: Cube = Cube::Green(13);
    const BLUE: Cube = Cube::Blue(14);

    games
        .iter()
        .filter(|game| game.is_possible((RED, GREEN, BLUE)))
        .map(|game| game.id)
        .sum()
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let games = Game::from_stdin(stdin)?;

    println!("Part one: {}", part_one(&games));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_from_str() -> Result<()> {
        assert_eq!(Cube::from_str("3 red")?, Cube::Red(3));
        assert_eq!(Cube::from_str("3 green")?, Cube::Green(3));
        assert_eq!(Cube::from_str("3 blue")?, Cube::Blue(3));

        Ok(())
    }

    #[test]
    fn cube_set_from_str() -> Result<()> {
        assert_eq!(
            CubeSet::from_str("3 blue, 4 red")?.cubes,
            vec![Cube::Blue(3), Cube::Red(4)],
        );
        assert_eq!(
            CubeSet::from_str("1 red, 2 green, 6 blue")?.cubes,
            vec![Cube::Red(1), Cube::Green(2), Cube::Blue(6)],
        );
        assert_eq!(CubeSet::from_str("2 green")?.cubes, vec![Cube::Green(2)]);

        Ok(())
    }

    #[test]
    fn game_from_str() -> Result<()> {
        let games = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .into_iter()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

        games
            .iter()
            .enumerate()
            .map(|(idx, game)| (idx + 1, game))
            .for_each(|(id, game)| assert_eq!(game.id, id));

        assert_eq!(
            games[0].cube_sets,
            vec![
                CubeSet {
                    cubes: vec![Cube::Blue(3), Cube::Red(4)],
                },
                CubeSet {
                    cubes: vec![Cube::Red(1), Cube::Green(2), Cube::Blue(6)],
                },
                CubeSet {
                    cubes: vec![Cube::Green(2)],
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn cube_is_possible() {
        assert!(Cube::Red(3).is_possible((Cube::Red(5), Cube::Green(0), Cube::Blue(0))));
        assert!(!Cube::Red(6).is_possible((Cube::Red(5), Cube::Green(0), Cube::Blue(0))));
    }

    #[test]
    fn cube_set_is_possible() {
        assert!(CubeSet {
            cubes: vec![Cube::Blue(3), Cube::Red(4)]
        }
        .is_possible((Cube::Red(12), Cube::Green(13), Cube::Blue(14))));
    }

    #[test]
    fn game_is_possible() {
        assert!(Game {
            id: 0,
            cube_sets: vec![CubeSet {
                cubes: vec![Cube::Blue(3), Cube::Red(4)]
            }],
        }
        .is_possible((Cube::Red(12), Cube::Green(13), Cube::Blue(14))));
    }
}
