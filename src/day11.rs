use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Image {
    galaxies: Vec<Pos>,
}

impl TryFrom<(usize, usize)> for Pos {
    type Error = Error;

    fn try_from((x, y): (usize, usize)) -> Result<Self> {
        Ok(Self {
            x: i32::try_from(x)?,
            y: i32::try_from(y)?,
        })
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(image: &str) -> Result<Self> {
        let mut grid = image
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = grid.first().map(Vec::len).unwrap_or_default();

        let cols_to_expand = (0..width)
            .filter(|&c| (0..height).all(|r| grid[r][c] == '.'))
            .collect::<Vec<_>>();
        let rows_to_expand = (0..height)
            .filter(|&r| grid[r].iter().all(|&tile| tile == '.'))
            .collect::<Vec<_>>();

        let height = height + rows_to_expand.len();
        let width = width + cols_to_expand.len();

        for c in cols_to_expand.iter().enumerate().map(|(i, c)| i + c) {
            for row in &mut grid {
                row.insert(c, '.');
            }
        }

        for r in rows_to_expand.iter().enumerate().map(|(i, r)| i + r) {
            grid.insert(r, vec!['.'; width]);
        }

        let galaxies = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|&(x, y)| grid[y][x] == '#')
            .map(|(x, y)| Pos::try_from((x, y)))
            .collect::<Result<_>>()?;

        Ok(Self { galaxies })
    }
}

impl Pos {
    const fn dist(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(image: &Image) -> i32 {
    let n = image.galaxies.len();
    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| image.galaxies[i].dist(image.galaxies[j])))
        .sum()
}

fn main() -> Result<()> {
    let image = Image::from_str(&fs::read_to_string("in/day11.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&image);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 9_522_407);
    };

    Ok(())
}
