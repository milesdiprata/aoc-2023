use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    network: Vec<(usize, usize)>,
    start: usize,
    end: usize,
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(instruction: char) -> Result<Self> {
        match instruction {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => bail!("invalid instruction '{instruction}'"),
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let mut parts = map.split("\n\n");

        let instructions = parts
            .next()
            .ok_or_else(|| anyhow!("missing instructions in map"))?
            .chars()
            .map(Instruction::try_from)
            .collect::<Result<Vec<_>>>()?;

        let network = parts
            .next()
            .ok_or_else(|| anyhow!("missing network in map"))?;
        let node_names = network
            .lines()
            .filter_map(|node| node.split(" = ").next())
            .collect::<Vec<_>>();
        let node_idxs = node_names
            .iter()
            .enumerate()
            .map(|(idx, &name)| (name, idx))
            .collect::<HashMap<_, _>>();

        let network = network
            .lines()
            .filter_map(|node| node.split(" = ").nth(1))
            .map(|dest| dest.trim_start_matches('(').trim_end_matches(')'))
            .map(|dest| dest.split(", "))
            .map(|mut split| split.next().zip(split.next()))
            .map(|split| split.ok_or_else(|| anyhow!("missing dest(s) in network")))
            .map(|split| split.map(|(i, j)| (node_idxs[i], node_idxs[j])))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            instructions,
            network,
            start: node_idxs["AAA"],
            end: node_idxs["ZZZ"],
        })
    }
}

fn part1(map: &Map) -> usize {
    let mut count = 0;

    let mut idx_network = map.start;
    let mut idx_instruction = 0;

    while idx_network != map.end {
        count += 1;

        idx_network = match map.instructions[idx_instruction] {
            Instruction::Left => map.network[idx_network].0,
            Instruction::Right => map.network[idx_network].1,
        };

        idx_instruction += 1;
        idx_instruction %= map.instructions.len();
    }

    count
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day8.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&map);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 11_911);
    };

    Ok(())
}
