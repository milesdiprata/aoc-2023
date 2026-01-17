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
    empty_rows: Vec<i32>,
    empty_cols: Vec<i32>,
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
        let grid = image
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = grid.first().map(Vec::len).unwrap_or_default();

        let galaxies = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|&(x, y)| grid[y][x] == '#')
            .map(|(x, y)| Pos::try_from((x, y)))
            .collect::<Result<_>>()?;
        let empty_rows = (0..height)
            .filter(|&r| grid[r].iter().all(|&tile| tile == '.'))
            .map(i32::try_from)
            .collect::<Result<_, _>>()?;
        let empty_cols = (0..width)
            .filter(|&c| (0..height).all(|r| grid[r][c] == '.'))
            .map(i32::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

impl Image {
    fn dist_expanded(&self, i: Pos, j: Pos, factor: i64) -> i64 {
        let (row_min, row_max) = (i.y.min(j.y), i.y.max(j.y));
        let (col_min, col_max) = (i.x.min(j.x), i.x.max(j.x));

        let dist = i64::from((row_max - row_min) + (col_max - col_min));

        let crossed_empty_rows = self
            .empty_rows
            .iter()
            .filter(|&&r| row_min < r && r < row_max)
            .count();
        let crossed_empty_cols = self
            .empty_cols
            .iter()
            .filter(|&&c| col_min < c && c < col_max)
            .count();

        let crossed_empty_rows = i64::try_from(crossed_empty_rows).unwrap_or_default();
        let crossed_empty_cols = i64::try_from(crossed_empty_cols).unwrap_or_default();

        dist + ((factor - 1) * (crossed_empty_rows + crossed_empty_cols))
    }
}

fn solve(image: &Image, factor: i64) -> i64 {
    let n = image.galaxies.len();
    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (image.galaxies[i], image.galaxies[j])))
        .map(|(i, j)| image.dist_expanded(i, j, factor))
        .sum()
}

fn main() -> Result<()> {
    let image = Image::from_str(&fs::read_to_string("in/day11.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::solve(&image, 2);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 9_522_407);
    };

    {
        let start = Instant::now();
        let part2 = self::solve(&image, 1_000_000);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 544_723_432_977);
    };

    Ok(())
}
