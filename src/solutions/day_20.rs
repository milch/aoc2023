use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;
use num::Integer;

use crate::utils::OptionFlatMap;

const INPUT: &str = include_str!("day_20.txt");

#[derive(Clone, Copy, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Clone)]
enum Module {
    FlipFlop(State),
    Conjunction(HashMap<String, State>),
    Broadcast,
}

struct Machine {
    parsed: HashMap<String, (Module, Vec<String>)>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = HashMap::from_iter(s.lines().map(|line| {
            let (module_description, adj_list) = line.split_once(" -> ").unwrap();
            let (label, module) = match module_description {
                "broadcaster" => ("broadcaster", Module::Broadcast),
                f if f.chars().nth(0) == Some('%') => {
                    (&f[1..f.len()], Module::FlipFlop(State::Off))
                }
                c if c.chars().nth(0) == Some('&') => {
                    (&c[1..c.len()], Module::Conjunction(HashMap::new()))
                }
                desc => unreachable!("Unhandled description: {desc}"),
            };
            (
                label.to_string(),
                (
                    module,
                    adj_list.split(", ").map(|s| s.to_string()).collect_vec(),
                ),
            )
        }));

        Ok(Machine { parsed })
    }
}

impl Machine {
    fn push_button(&self, times: usize, search_for_rx: bool) -> usize {
        let mut states: HashMap<&String, Module> = HashMap::from_iter(
            self.parsed
                .iter()
                .map(|(k, (module, _))| (k, module.clone())),
        );
        let adjacencies: HashMap<&String, &Vec<String>> =
            HashMap::from_iter(self.parsed.iter().map(|(k, (_, list))| (k, list)));
        let inputs = self
            .parsed
            .iter()
            .flat_map(|(k, (_, list))| list.iter().map(|input| (input, k)).collect_vec())
            .into_group_map();
        let mut pulses = (0, 0);

        let initial_recipient = "broadcaster".to_string();
        let initial_sender = "button".to_string();
        let default_destinations = vec![];
        let mut queue: VecDeque<(State, &String, &String)> = VecDeque::from([]);

        let rx_input = inputs
            .get(&"rx".to_string())
            .flat_map(|x| x.first())
            .cloned();
        let mut results: Option<HashMap<String, usize>> = rx_input.map(|rx_input| {
            inputs[rx_input]
                .iter()
                .flat_map(|input| inputs[input].clone())
                .cloned()
                .map(|x| (x, 0))
                .collect()
        });
        let rx_inputs: Option<HashSet<_>> = rx_input.map(|rx_input| {
            inputs[rx_input]
                .iter()
                .flat_map(|input| inputs[input].clone())
                .collect()
        });

        for press in 1..=times {
            queue.push_back((State::Off, &initial_recipient, &initial_sender));
            while let Some((signal, recipient, sender)) = queue.pop_front() {
                match signal {
                    State::On => pulses.0 += 1,
                    State::Off => pulses.1 += 1,
                }
                let destinations = *adjacencies
                    .get(&recipient)
                    .unwrap_or(&&default_destinations);
                let receiver_module = states.get_mut(&recipient);
                let new_state = match receiver_module {
                    Some(Module::FlipFlop(state)) => {
                        if matches!(signal, State::Off) {
                            let new_state = match state {
                                State::On => State::Off,
                                State::Off => State::On,
                            };
                            states.insert(recipient, Module::FlipFlop(new_state));
                            Some(new_state)
                        } else {
                            None
                        }
                    }
                    Some(Module::Conjunction(ref mut memory)) => {
                        memory.insert(sender.clone(), signal);
                        if inputs[recipient].iter().all(|&input| {
                            memory.entry(input.clone()).or_insert(State::Off);
                            matches!(memory[input], State::On)
                        }) {
                            if search_for_rx {
                                let rx_inputs = rx_inputs.as_ref().unwrap();
                                let results = results.as_mut().unwrap();
                                if rx_inputs.contains(recipient) {
                                    results.insert(recipient.clone(), press);

                                    if results.values().all(|&v| v != 0) {
                                        let lcm =
                                            results.values().cloned().reduce(|a, b| a.lcm(&b));
                                        return lcm.unwrap();
                                    }
                                }
                            }
                            Some(State::Off)
                        } else {
                            Some(State::On)
                        }
                    }
                    Some(Module::Broadcast) => Some(signal),
                    None => None,
                };
                if let Some(new_state) = new_state {
                    for destination in destinations {
                        queue.push_back((new_state, destination, recipient));
                    }
                }
            }
        }

        pulses.0 * pulses.1
    }
}

pub fn print_solution() {
    let machine: Machine = INPUT.parse().unwrap();
    println!("Pulses: {}", machine.push_button(1000, false));
    println!(
        "Button presses to reach rx: {}",
        machine.push_button(usize::MAX, true)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SIMPLE_SAMPLE: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    const COMPLEX_SAMPLE: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn test_pushing_the_button() {
        let simple: Machine = SIMPLE_SAMPLE.parse().unwrap();
        assert_eq!(simple.push_button(1000, false), 32000000);

        let complex: Machine = COMPLEX_SAMPLE.parse().unwrap();
        assert_eq!(complex.push_button(1000, false), 11687500);
    }
}
