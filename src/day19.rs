use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
enum Category {
    Extremely,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Clone, Copy, Debug)]
enum Comparison {
    Less,
    Greater,
}

#[derive(Debug)]
enum Rule {
    If {
        category: Category,
        cmp: Comparison,
        val: u32,
        workflow: String,
    },
    Else {
        workflow: String,
    },
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Part {
    ratings: [u32; 4],
}

impl TryFrom<char> for Category {
    type Error = Error;

    fn try_from(category: char) -> Result<Self> {
        match category {
            'x' => Ok(Self::Extremely),
            'm' => Ok(Self::Musical),
            'a' => Ok(Self::Aerodynamic),
            's' => Ok(Self::Shiny),
            _ => bail!("invalid category '{category}'"),
        }
    }
}

impl TryFrom<char> for Comparison {
    type Error = Error;

    fn try_from(cond: char) -> Result<Self> {
        match cond {
            '<' => Ok(Self::Less),
            '>' => Ok(Self::Greater),
            _ => bail!("invalid condition '{cond}'"),
        }
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        if rule.chars().all(char::is_alphabetic) {
            Ok(Self::Else {
                workflow: rule.to_string(),
            })
        } else {
            let mut rule = rule.split(':');

            let condition = rule
                .next()
                .ok_or_else(|| anyhow!("missing condition in rule"))?;
            let workflow = rule
                .next()
                .ok_or_else(|| anyhow!("missing workflow in rule"))?
                .to_string();

            let category = condition
                .chars()
                .next()
                .ok_or_else(|| anyhow!("missing category in rule condition"))?
                .try_into()?;
            let cmp = condition
                .chars()
                .nth(1)
                .ok_or_else(|| anyhow!("missing comparison in rule condition"))?
                .try_into()?;
            let val = condition
                .get(2..)
                .ok_or_else(|| anyhow!("missing comparison value in rule condition"))?
                .parse()?;

            Ok(Self::If {
                category,
                cmp,
                val,
                workflow,
            })
        }
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(workflow: &str) -> Result<Self> {
        let name = workflow
            .chars()
            .take_while(|&c| c.is_alphabetic())
            .collect::<String>();
        let rules = workflow
            .get(name.len()..)
            .ok_or_else(|| anyhow!("missing workflow rules"))?
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(str::parse)
            .collect::<Result<_>>()?;

        Ok(Self { name, rules })
    }
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(part: &str) -> Result<Self> {
        let mut ratings = part
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',');

        let x = ratings
            .next()
            .ok_or_else(|| anyhow!("missing part 'x' rating"))?
            .split("x=")
            .nth(1)
            .ok_or_else(|| anyhow!("missing 'x='"))?
            .parse()?;
        let m = ratings
            .next()
            .ok_or_else(|| anyhow!("missing part 'm' rating"))?
            .split("m=")
            .nth(1)
            .ok_or_else(|| anyhow!("missing 'm='"))?
            .parse()?;
        let a = ratings
            .next()
            .ok_or_else(|| anyhow!("missing part 'a' rating"))?
            .split("a=")
            .nth(1)
            .ok_or_else(|| anyhow!("missing 'a='"))?
            .parse()?;
        let s = ratings
            .next()
            .ok_or_else(|| anyhow!("missing part 's' rating"))?
            .split("s=")
            .nth(1)
            .ok_or_else(|| anyhow!("missing 's='"))?
            .parse()?;

        Ok(Self {
            ratings: [x, m, a, s],
        })
    }
}

impl Category {
    const fn idx(self) -> usize {
        match self {
            Self::Extremely => 0,
            Self::Musical => 1,
            Self::Aerodynamic => 2,
            Self::Shiny => 3,
        }
    }
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<&str> {
        match self {
            Self::If {
                category,
                cmp,
                val,
                workflow,
            } => match cmp {
                Comparison::Less => part.ratings[category.idx()] < *val,
                Comparison::Greater => part.ratings[category.idx()] > *val,
            }
            .then_some(workflow.as_str()),
            Self::Else { workflow } => Some(workflow.as_str()),
        }
    }
}

fn parse() -> Result<(Vec<Workflow>, Vec<Part>)> {
    let input = fs::read_to_string("in/day19.txt")?;
    let mut input = input.split("\n\n");

    let workflows = input
        .next()
        .ok_or_else(|| anyhow!("missing workflows"))?
        .lines()
        .map(Workflow::from_str)
        .collect::<Result<_>>()?;
    let parts = input
        .next()
        .ok_or_else(|| anyhow!("missing part ratings"))?
        .lines()
        .map(Part::from_str)
        .collect::<Result<_>>()?;

    Ok((workflows, parts))
}

fn part1(workflows: &[Workflow], parts: &[Part]) -> u32 {
    let workflows = workflows
        .iter()
        .map(|workflow| (workflow.name.as_str(), workflow))
        .collect::<HashMap<_, _>>();

    let mut accepted = vec![];
    for part in parts {
        let mut workflow = "in";

        while workflow != "A" && workflow != "R" {
            for rule in &workflows[workflow].rules {
                if let Some(next) = rule.evaluate(part) {
                    workflow = next;
                    break;
                }
            }
        }

        if workflow == "A" {
            accepted.push(part);
        }
    }

    accepted
        .into_iter()
        .map(|part| part.ratings.iter().copied().sum::<u32>())
        .sum()
}

#[allow(clippy::similar_names)]
fn main() -> Result<()> {
    let (workflows, parts) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&workflows, &parts);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 376_008);
    };

    Ok(())
}
