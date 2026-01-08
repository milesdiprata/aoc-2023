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

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
struct SeedRange {
    start: u32,
    len: u32,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_ranges: Vec<SeedRange>,
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

        let seed_ranges = seeds
            .chunks(2)
            .map(|chunk| SeedRange {
                start: chunk[0],
                len: chunk[1],
            })
            .collect();

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
            seed_ranges,
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
    const fn map(&self, src: u32) -> Option<u32> {
        if src >= self.start_src && src < self.start_src + self.len {
            Some(self.start_dest + (src - self.start_src))
        } else {
            None
        }
    }

    fn map_range(&self, input: &SeedRange) -> (Option<SeedRange>, Vec<SeedRange>) {
        let end = self.start_src + self.len;
        let end_input = input.start + input.len;

        if input.start >= end || end_input <= self.start_src {
            // No overlap
            return (None, vec![input.clone()]);
        }

        let mut unmapped = Vec::new();

        if input.start < self.start_src {
            // Unmapped ranged before start
            unmapped.push(SeedRange {
                start: input.start,
                len: self.start_src - input.start,
            });
        }

        if end_input > end {
            // Unmapped range after end
            unmapped.push(SeedRange {
                start: end,
                len: end_input - end,
            });
        }

        let start_mapped = self.start_src.max(input.start);
        let end_mapped = end.min(end_input);
        let mapped = SeedRange {
            start: self.start_dest + (start_mapped - self.start_src),
            len: end_mapped - start_mapped,
        };

        (Some(mapped), unmapped)
    }
}

impl Almanac {
    fn map_seeds(&self) -> Vec<u32> {
        let mut results = Vec::with_capacity(self.seeds.len());

        for &seed in &self.seeds {
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
                result = Self::map(result, ranges);
            }

            results.push(result);
        }

        results
    }

    fn map_seed_ranges(&self) -> Vec<SeedRange> {
        let mut seed_ranges = self.seed_ranges.clone();

        for ranges in [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ] {
            seed_ranges = Self::map_range(seed_ranges, ranges);
        }

        seed_ranges
    }

    fn map(src: u32, ranges: &[Range]) -> u32 {
        ranges
            .iter()
            .find_map(|range| range.map(src))
            .unwrap_or(src)
    }

    fn map_range(input: Vec<SeedRange>, ranges: &[Range]) -> Vec<SeedRange> {
        let mut unmapped = input;
        let mut mapped = Vec::new();

        for range in ranges {
            let mut unmapped_still = Vec::new();

            for u in unmapped {
                let (m, u) = range.map_range(&u);
                if let Some(m) = m {
                    mapped.push(m);
                }

                unmapped_still.extend(u);
            }

            unmapped = unmapped_still;
        }

        mapped.extend(unmapped);
        mapped
    }
}

fn part1(almanac: &Almanac) -> u32 {
    almanac.map_seeds().into_iter().min().unwrap_or_default()
}

fn part2(almanac: &Almanac) -> u32 {
    almanac
        .map_seed_ranges()
        .into_iter()
        .map(|range| range.start)
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

    {
        let start = Instant::now();
        let part2 = self::part2(&almanac);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 60_294_664);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_map() {
        let range = Range {
            start_src: 98,
            start_dest: 50,
            len: 2,
        };

        assert_eq!(range.map(97), None);
        assert_eq!(range.map(98), Some(50));
        assert_eq!(range.map(99), Some(51));
        assert_eq!(range.map(100), None);
    }

    #[test]
    fn range_map_range() {
        let range = Range {
            start_src: 98,
            start_dest: 50,
            len: 2,
        };

        let (mapped, unmapped) = range.map_range(&SeedRange { start: 97, len: 4 });

        assert_eq!(mapped, Some(SeedRange { start: 50, len: 2 }));
        assert_eq!(
            unmapped,
            [
                SeedRange { start: 97, len: 1 },
                SeedRange { start: 100, len: 1 }
            ]
        );
    }

    #[test]
    fn almanac_map() {
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
            assert_eq!(Almanac::map(src, &ranges), src);
        }

        for src in 50..=97 {
            assert_eq!(Almanac::map(src, &ranges), src + 2);
        }

        assert_eq!(Almanac::map(98, &ranges), 50);
        assert_eq!(Almanac::map(99, &ranges), 51);
        assert_eq!(Almanac::map(100, &ranges), 100);
    }
}
