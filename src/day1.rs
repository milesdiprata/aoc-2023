use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::Stdin;
use std::str;
use std::str::Chars;

use anyhow::anyhow;
use anyhow::Result;

trait FromWord<T> {
    fn from_word(word: &str) -> Option<T>;
}

#[derive(Debug)]
struct Calibration {
    raw: String,
}

#[derive(Debug)]
struct Trebuchet {
    calibrations: Vec<Calibration>,
}

impl FromWord<Self> for u32 {
    fn from_word(word: &str) -> Option<Self> {
        [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .flat_map(|name| word.find(name).map(|idx| (name, idx)))
        .min_by(|(_, idx_a), (_, idx_b)| idx_a.cmp(idx_b))
        .map(|(digit, _)| digit)
        .and_then(|digit| match digit {
            "1" | "one" => Some(1),
            "2" | "two" => Some(2),
            "3" | "three" => Some(3),
            "4" | "four" => Some(4),
            "5" | "five" => Some(5),
            "6" | "six" => Some(6),
            "7" | "seven" => Some(7),
            "8" | "eight" => Some(8),
            "9" | "nine" => Some(9),
            _ => None,
        })
    }
}

impl Calibration {
    const RADIX: u32 = 10;

    fn from_raw(raw: String) -> Self {
        Self { raw }
    }

    fn value(&self) -> Option<u32> {
        let nums = self
            .raw
            .chars()
            .filter_map(|char| char.to_digit(Self::RADIX));

        let first = nums.clone().next()?;
        let last = nums.rev().next()?;

        Some((Self::RADIX * first) + last)
    }

    fn value2(&self) -> Option<u32> {
        const WINDOW_LEN: usize = 5;

        let nums = self
            .raw
            .char_indices()
            .map(|(idx, _)| &self.raw[idx..])
            .filter_map(u32::from_word);

        let first = nums.clone().next()?;
        let last = nums.rev().next()?;

        Some((Self::RADIX * first) + last)
    }
}

impl Trebuchet {
    fn from_stdin(stdin: Stdin) -> Result<Self> {
        let calibrations = stdin
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
            .map(Calibration::from_raw)
            .collect();

        Ok(Self { calibrations })
    }

    fn value(&self) -> Option<u32> {
        self.calibrations.iter().map(Calibration::value).sum()
    }

    fn value2(&self) -> Option<u32> {
        self.calibrations.iter().map(Calibration::value2).sum()
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let trebuchet = Trebuchet::from_stdin(stdin)?;

    println!("{trebuchet:#?}");

    println!("Part one: {:?}", trebuchet.value());
    println!("Part two: {:?}", trebuchet.value2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_value() {
        assert_eq!(
            Calibration {
                raw: "1abc2".to_string()
            }
            .value()
            .unwrap_or_default(),
            12,
        );
        assert_eq!(
            Calibration {
                raw: "pqr3stu8vwx".to_string()
            }
            .value()
            .unwrap_or_default(),
            38,
        );
        assert_eq!(
            Calibration {
                raw: "a1b2c3d4e5f".to_string()
            }
            .value()
            .unwrap_or_default(),
            15,
        );
        assert_eq!(
            Calibration {
                raw: "treb7uchet".to_string()
            }
            .value()
            .unwrap_or_default(),
            77,
        );
    }

    #[test]
    fn calibration_value2() {
        assert_eq!(
            Calibration {
                raw: "two1nine".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            29,
        );
        assert_eq!(
            Calibration {
                raw: "eightwothree".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            83,
        );
        assert_eq!(
            Calibration {
                raw: "abcone2threexyz".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            13,
        );
        assert_eq!(
            Calibration {
                raw: "xtwone3four".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            24,
        );
        assert_eq!(
            Calibration {
                raw: "4nineeightseven2".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            42,
        );
        assert_eq!(
            Calibration {
                raw: "zoneight234".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            14,
        );
        assert_eq!(
            Calibration {
                raw: "7pqrstsixteen".to_string(),
            }
            .value2()
            .unwrap_or_default(),
            76,
        );
    }

    #[test]
    fn trebuchet_value() {
        let trebuchet = Trebuchet {
            calibrations: vec![
                Calibration {
                    raw: "1abc2".to_string(),
                },
                Calibration {
                    raw: "pqr3stu8vwx".to_string(),
                },
                Calibration {
                    raw: "a1b2c3d4e5f".to_string(),
                },
                Calibration {
                    raw: "treb7uchet".to_string(),
                },
            ],
        };

        assert_eq!(trebuchet.value().unwrap_or_default(), 142);
    }

    #[test]
    fn trebuchet_value2() {
        let trebuchet = Trebuchet {
            calibrations: vec![
                Calibration {
                    raw: "two1nine".to_string(),
                },
                Calibration {
                    raw: "eightwothree".to_string(),
                },
                Calibration {
                    raw: "abcone2threexyz".to_string(),
                },
                Calibration {
                    raw: "xtwone3four".to_string(),
                },
                Calibration {
                    raw: "4nineeightseven2".to_string(),
                },
                Calibration {
                    raw: "zoneight234".to_string(),
                },
                Calibration {
                    raw: "7pqrstsixteen".to_string(),
                },
            ],
        };

        assert_eq!(trebuchet.value2().unwrap_or_default(), 281);
    }
}
