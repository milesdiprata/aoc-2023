use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl FromStr for Hailstone {
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

impl Hailstone {
    fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        // Cramer's rule
        let det = other.vx.mul_add(self.vy, -(self.vx * other.vy));
        if det == 0.0 {
            // Parallel paths
            return None;
        }

        let dx = other.px - self.px;
        let dy = other.py - self.py;

        let ta = other.vx.mul_add(dy, -(other.vy * dx)) / det;
        let tb = self.vx.mul_add(dy, -(self.vy * dx)) / det;

        if ta < 0.0 || tb < 0.0 {
            // Intersection in the past
            return None;
        }

        let ix = self.vx.mul_add(ta, self.px);
        let iy = self.vy.mul_add(ta, self.py);

        Some((ix, iy))
    }
}

fn part1(hailstones: &[Hailstone], min: f64, max: f64) -> usize {
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

fn main() -> Result<()> {
    let hailstones = fs::read_to_string("in/day24.txt")?
        .lines()
        .map(Hailstone::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        const MIN: f64 = 200e12;
        const MAX: f64 = 400e12;

        let start = Instant::now();
        let part1 = self::part1(&hailstones, MIN, MAX);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 17_244);
    };

    Ok(())
}
