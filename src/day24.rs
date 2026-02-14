use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug, Default)]
struct Stone {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

impl FromStr for Stone {
    type Err = Error;

    fn from_str(hailstone: &str) -> Result<Self> {
        let mut parts = hailstone
            .split('@')
            .map(|part| part.split(','))
            .map(|vals| vals.map(str::trim).map(str::parse));

        let mut pos = parts
            .next()
            .ok_or_else(|| anyhow!("missing hailstone position"))?;
        let mut vel = parts
            .next()
            .ok_or_else(|| anyhow!("missing hailstone velocity"))?;

        let px = pos
            .next()
            .ok_or_else(|| anyhow!("missing hailstone x-position"))??;
        let py = pos
            .next()
            .ok_or_else(|| anyhow!("missing hailstone y-position"))??;
        let pz = pos
            .next()
            .ok_or_else(|| anyhow!("missing hailstone z-position"))??;

        let vx = vel
            .next()
            .ok_or_else(|| anyhow!("missing hailstone x-velocity"))??;
        let vy = vel
            .next()
            .ok_or_else(|| anyhow!("missing hailstone y-velocity"))??;
        let vz = vel
            .next()
            .ok_or_else(|| anyhow!("missing hailstone z-velocity"))??;

        Ok(Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        })
    }
}

impl Stone {
    const fn adjusted(&self, dvx: i128, dvy: i128) -> Self {
        Self {
            px: self.px,
            py: self.py,
            pz: self.pz,
            vx: self.vx - dvx,
            vy: self.vy - dvy,
            vz: self.vz,
        }
    }

    const fn intersection(&self, other: &Self) -> Option<(i128, i128)> {
        // Cramer's rule
        let det = (other.vx * self.vy) - (self.vx * other.vy);
        if det == 0 {
            // Parallel paths
            return None;
        }

        let dx = other.px - self.px;
        let dy = other.py - self.py;

        let ta = ((other.vx * dy) - (other.vy * dx)) / det;
        let tb = ((self.vx * dy) - (self.vy * dx)) / det;

        if ta < 0 || tb < 0 {
            // Intersection in the past
            return None;
        }

        let ix = (self.vx * ta) + self.px;
        let iy = (self.vy * ta) + self.py;

        Some((ix, iy))
    }

    const fn intersection_exact(&self, other: &Self) -> Option<(i128, i128, i128, i128)> {
        // Cramer's rule, requiring exact integer times
        let det = (other.vx * self.vy) - (self.vx * other.vy);
        if det == 0 {
            return None;
        }

        let dx = other.px - self.px;
        let dy = other.py - self.py;

        let ta_numerator = (other.vx * dy) - (other.vy * dx);
        let tb_numerator = (self.vx * dy) - (self.vy * dx);

        if ta_numerator % det != 0 || tb_numerator % det != 0 {
            return None;
        }

        let ta = ta_numerator / det;
        let tb = tb_numerator / det;

        if ta < 0 || tb < 0 {
            return None;
        }

        let ix = (self.vx * ta) + self.px;
        let iy = (self.vy * ta) + self.py;

        Some((ix, iy, ta, tb))
    }

    const fn time_to_point(&self, x: i128, y: i128) -> Option<i128> {
        let (numerator, denominator) = if self.vx != 0 {
            (x - self.px, self.vx)
        } else if self.vy != 0 {
            (y - self.py, self.vy)
        } else if self.px == x && self.py == y {
            return Some(0);
        } else {
            return None;
        };

        if numerator % denominator != 0 {
            return None;
        }

        let t = numerator / denominator;
        if t < 0 || self.px + self.vx * t != x || self.py + self.vy * t != y {
            return None;
        }

        Some(t)
    }
}

fn part1(hailstones: &[Stone], min: i128, max: i128) -> usize {
    let mut count = 0;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if hailstones[i]
                .intersection(&hailstones[j])
                .is_some_and(|(x, y)| min <= x && x <= max && min <= y && y <= max)
            {
                count += 1;
            }
        }
    }

    count
}

fn part2(hailstones: &[Stone]) -> Option<i128> {
    // Brute force: search for the rock's velocity (rvx, rvy).
    // In the rock's reference frame, the rock is stationary, so all hailstones
    // must pass through the same point. We adjust each hailstone's velocity by
    // subtracting (rvx, rvy), then check if all paths intersect at one point.

    const RANGE: i128 = 500;

    for rvx in -RANGE..=RANGE {
        'next: for rvy in -RANGE..=RANGE {
            let a = hailstones[0].adjusted(rvx, rvy);
            let b = hailstones[1].adjusted(rvx, rvy);

            let Some((rpx, rpy, ta, tb)) = a.intersection_exact(&b) else {
                continue;
            };

            // Verify all other hailstones pass through (rpx, rpy)
            if hailstones[2..]
                .iter()
                .any(|s| s.adjusted(rvx, rvy).time_to_point(rpx, rpy).is_none())
            {
                continue 'next;
            }

            // Solve for rvz and rpz using the z-coordinates at times ta and tb
            let za = hailstones[0].pz + hailstones[0].vz * ta;
            let zb = hailstones[1].pz + hailstones[1].vz * tb;
            let rvz = (za - zb) / (ta - tb);
            let rpz = za - rvz * ta;

            return Some(rpx + rpy + rpz);
        }
    }

    None
}

fn main() -> Result<()> {
    let hailstones = fs::read_to_string("in/day24.txt")?
        .lines()
        .map(Stone::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        const MIN: i128 = 200_000_000_000_000;
        const MAX: i128 = 400_000_000_000_000;

        let start = Instant::now();
        let part1 = self::part1(&hailstones, MIN, MAX);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 17_244);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&hailstones).ok_or_else(|| anyhow!("no solution found in rage"))?;
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_025_019_997_186_820);
    };

    Ok(())
}
