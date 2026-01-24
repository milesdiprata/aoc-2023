use std::fs;
use std::time::Instant;

use anyhow::Result;

#[allow(clippy::cast_possible_truncation)]
fn hash(step: &str) -> u8 {
    let mut val = 0_u32;

    for &byte in step.as_bytes() {
        val += u32::from(byte);
        val *= 17;
        val &= 255;
    }

    val as u8
}

fn part1(seq: &[String]) -> u32 {
    seq.iter().map(|step| self::hash(step)).map(u32::from).sum()
}

fn main() -> Result<()> {
    let seq = fs::read_to_string("in/day15.txt")?
        .replace('\n', "")
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();

    {
        let start = Instant::now();
        let part1 = self::part1(&seq);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 502_139);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
    }
}
