use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Range {
    start_src: u32,
    start_dest: u32,
    len: u32,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let mut parts = map.split_ascii_whitespace();

        let start_dest = parts
            .next()
            .ok_or_else(|| anyhow!("missing destination start"))?
            .parse()?;
        let start_src = parts
            .next()
            .ok_or_else(|| anyhow!("missing source start"))?
            .parse()?;
        let len = parts
            .next()
            .ok_or_else(|| anyhow!("missing range length"))?
            .parse()?;

        Ok(Self {
            start_src,
            start_dest,
            len,
        })
    }
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(almanac: &str) -> Result<Self> {
        let mut parts = almanac.split("\n\n");

        let seeds = parts
            .next()
            .ok_or_else(|| anyhow!("missing seeds in almanac"))?
            .split("seeds: ")
            .nth(1)
            .ok_or_else(|| anyhow!("missing seed numbers"))?
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        let mut maps = {
            let map_names = [
                "seed-to-soil",
                "soil-to-fertilizer",
                "fertilizer-to-water",
                "water-to-light",
                "light-to-temperature",
                "temperature-to-humidity",
                "humidity-to-location ",
            ];

            let mut maps = Vec::with_capacity(map_names.len());

            for map in map_names {
                maps.push(
                    parts
                        .next()
                        .ok_or_else(|| anyhow!(format!("missing {map} map in almanac")))?
                        .lines()
                        .skip(1)
                        .map(Range::from_str)
                        .collect::<Result<Vec<_>>>()?,
                );
            }

            maps.into_iter()
        };

        Ok(Self {
            seeds,
            seed_to_soil: maps.next().unwrap_or_else(|| unreachable!()),
            soil_to_fertilizer: maps.next().unwrap_or_else(|| unreachable!()),
            fertilizer_to_water: maps.next().unwrap_or_else(|| unreachable!()),
            water_to_light: maps.next().unwrap_or_else(|| unreachable!()),
            light_to_temperature: maps.next().unwrap_or_else(|| unreachable!()),
            temperature_to_humidity: maps.next().unwrap_or_else(|| unreachable!()),
            humidity_to_location: maps.next().unwrap_or_else(|| unreachable!()),
        })
    }
}

impl Range {
    const fn dest(&self, src: u32) -> Option<u32> {
        if src >= self.start_src && src < self.start_src + self.len {
            Some(self.start_dest + (src - self.start_src))
        } else {
            None
        }
    }
}

impl Almanac {
    fn seed_to_location(&self, seed: u32) -> u32 {
        let mut result = seed;

        for ranges in [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ] {
            result = Self::find_dest(result, ranges);
        }

        result
    }

    fn find_dest(src: u32, ranges: &[Range]) -> u32 {
        ranges
            .iter()
            .find_map(|range| range.dest(src))
            .unwrap_or(src)
    }
}

fn part1(almanac: &Almanac) -> u32 {
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let almanac = Almanac::from_str(&fs::read_to_string("in/day5.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&almanac);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 346_433_842);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_dest() {
        let range = Range {
            start_src: 98,
            start_dest: 50,
            len: 2,
        };

        assert_eq!(range.dest(97), None);
        assert_eq!(range.dest(98), Some(50));
        assert_eq!(range.dest(99), Some(51));
        assert_eq!(range.dest(100), None);
    }

    #[test]
    fn almanac_find_dest() {
        let ranges = [
            Range {
                start_src: 98,
                start_dest: 50,
                len: 2,
            },
            Range {
                start_src: 50,
                start_dest: 52,
                len: 48,
            },
        ];

        for src in 0..=49 {
            assert_eq!(Almanac::find_dest(src, &ranges), src);
        }

        for src in 50..=97 {
            assert_eq!(Almanac::find_dest(src, &ranges), src + 2);
        }

        assert_eq!(Almanac::find_dest(98, &ranges), 50);
        assert_eq!(Almanac::find_dest(99, &ranges), 51);
        assert_eq!(Almanac::find_dest(100, &ranges), 100);
    }
}
