use std::fs;
use std::time::Instant;

use anyhow::Result;

#[derive(Debug)]
struct Document {
    text: String,
}

impl Document {
    fn vals_from_digits(&self) -> Vec<u8> {
        let mut vals = Vec::with_capacity(self.text.lines().count());

        for line in self.text.lines() {
            let line = line.as_bytes();

            let mut i = 0;
            while i < line.len() && !line[i].is_ascii_digit() {
                i += 1;
            }
            let first = line[i] - b'0';

            let mut i = line.len();
            while i > 0 && !line[i - 1].is_ascii_digit() {
                i -= 1;
            }
            let last = line[i - 1] - b'0';

                vals.push((10 * first) + last);
            }
        }

        vals
    }
}

fn part1(doc: &Document) -> u64 {
    doc.vals().into_iter().map(u64::from).sum()
}

fn main() -> Result<()> {
    let doc = Document {
        text: fs::read_to_string("in/day1.txt")?,
    };

    {
        let start = Instant::now();
        let part1 = self::part1(&doc);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 54_601);
    }

    Ok(())
}
