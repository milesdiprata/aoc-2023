use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    FlipFlop {
        name: String,
        state: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        mem: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
    Broadcast {
        outputs: Vec<String>,
    },
}

#[derive(Debug)]
struct ModuleConfig {
    modules: HashMap<String, Module>,
}

#[derive(Debug)]
struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(module: &str) -> Result<Self> {
        let mut module = module.split(" -> ");

        let lhs = module.next().ok_or_else(|| anyhow!("missing module LHS"))?;
        let outputs = module
            .next()
            .ok_or_else(|| anyhow!("missing module RHS"))?
            .split(", ")
            .map(str::to_string)
            .collect::<Vec<String>>();

        if lhs.starts_with('%') {
            Ok(Self::FlipFlop {
                name: lhs.chars().skip(1).collect(),
                state: false,
                outputs,
            })
        } else if lhs.starts_with('&') {
            Ok(Self::Conjunction {
                name: lhs.chars().skip(1).collect(),
                mem: HashMap::new(),
                outputs,
            })
        } else if lhs == "broadcaster" {
            Ok(Self::Broadcast { outputs })
        } else {
            bail!("invalid module '{lhs}'")
        }
    }
}

impl FromStr for ModuleConfig {
    type Err = Error;

    fn from_str(config: &str) -> Result<Self> {
        let modules = config
            .lines()
            .map(Module::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self::new(modules))
    }
}

impl Module {
    const fn name(&self) -> &str {
        match self {
            Self::FlipFlop { name, .. } | Self::Conjunction { name, .. } => name.as_str(),
            Self::Broadcast { .. } => "broadcast",
        }
    }

    const fn outputs(&self) -> &[String] {
        match self {
            Self::FlipFlop { outputs, .. }
            | Self::Conjunction { outputs, .. }
            | Self::Broadcast { outputs } => outputs.as_slice(),
        }
    }

    fn process(&mut self, from: &str, pulse: Pulse) -> Vec<Signal> {
        let pulse = match self {
            Self::FlipFlop { state, .. } => match pulse {
                Pulse::Low => {
                    let pulse = if *state { Pulse::Low } else { Pulse::High };
                    *state = !*state;
                    Some(pulse)
                }
                Pulse::High => None,
            },
            Self::Conjunction { mem, .. } => {
                *mem.get_mut(from).unwrap() = pulse;
                Some(if mem.values().all(|&pulse| pulse == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            Self::Broadcast { .. } => Some(pulse),
        };

        pulse
            .map(|pulse| {
                self.outputs()
                    .iter()
                    .cloned()
                    .map(|to| (self.name().to_string(), to))
                    .map(move |(from, to)| Signal { from, to, pulse })
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl ModuleConfig {
    fn new(mut modules: Vec<Module>) -> Self {
        let mut all_inputs = HashMap::new();
        for module in &modules {
            for output in module.outputs() {
                all_inputs
                    .entry(output.clone())
                    .or_insert_with(Vec::new)
                    .push(module.name().to_string());
            }
        }

        for module in &mut modules {
            match module {
                Module::Conjunction { name, mem, .. } => {
                    *mem = all_inputs
                        .remove(name.as_str())
                        .into_iter()
                        .flatten()
                        .map(|name| (name, Pulse::default()))
                        .collect();
                }
                Module::FlipFlop { .. } | Module::Broadcast { .. } => (),
            }
        }

        Self {
            modules: modules
                .into_iter()
                .map(|module| (module.name().to_string(), module))
                .collect(),
        }
    }

    fn press_button(&mut self) -> (usize, usize) {
        let mut presses_low = 0;
        let mut presses_high = 0;

        let mut queue = VecDeque::from([Signal {
            from: "button".to_string(),
            to: "broadcast".to_string(),
            pulse: Pulse::Low,
        }]);

        while let Some(signal) = queue.pop_front() {
            match signal.pulse {
                Pulse::Low => presses_low += 1,
                Pulse::High => presses_high += 1,
            }

            if let Some(module) = self.modules.get_mut(&signal.to) {
                for signal in module.process(&signal.from, signal.pulse) {
                    queue.push_back(signal);
                }
            }
        }

        (presses_low, presses_high)
    }
}

fn part1(config: &mut ModuleConfig) -> usize {
    let (presses_low, presses_high) = (0..1_000)
        .map(|_| config.press_button())
        .fold((0, 0), |(l_sum, h_sum), (l, h)| (l_sum + l, h_sum + h));

    presses_low * presses_high
}

fn main() -> Result<()> {
    let mut config = ModuleConfig::from_str(&fs::read_to_string("in/day20.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&mut config);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 819_397_964);
    };

    Ok(())
}
