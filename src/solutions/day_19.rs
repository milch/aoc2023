use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

use itertools::{FoldWhile, Itertools};

use crate::utils::{RangeLen, RangeSplit};

const INPUT: &str = include_str!("day_19.txt");

#[derive(Clone)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for Destination {
    fn from(value: &str) -> Destination {
        match value {
            "R" => Self::Reject,
            "A" => Self::Accept,
            s => Self::Workflow(s.to_owned()),
        }
    }
}
struct Operation {
    key: char,
    operator: char,
    rhs_operand: usize,
}

struct RuleInstruction {
    destination: Destination,

    operation: Option<Operation>,
}

impl RuleInstruction {
    fn destination(destination: Destination) -> RuleInstruction {
        RuleInstruction {
            operation: None,
            destination,
        }
    }

    fn operation(operation: Operation, destination: Destination) -> RuleInstruction {
        RuleInstruction {
            operation: Some(operation),
            destination,
        }
    }

    fn condition(&self, map: &HashMap<char, usize>) -> bool {
        match &self.operation {
            Some(op) => {
                let value = map.get(&op.key).unwrap();
                match op.operator {
                    '>' => value > &op.rhs_operand,
                    '<' => value < &op.rhs_operand,
                    _ => unreachable!("Unsupported operator"),
                }
            }
            None => true,
        }
    }
}

impl FromStr for RuleInstruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // {a<2006:qkq}
        match s {
            "R" | "A" => Ok(RuleInstruction::destination(s.into())),
            s if s.contains(':') => {
                let (condition, destination) = s.split_once(':').unwrap();
                let key = condition.chars().nth(0).unwrap();
                let operator = condition.chars().nth(1).unwrap();
                let rhs_operand = condition[2..condition.len()].parse()?;
                Ok(RuleInstruction::operation(
                    Operation {
                        key,
                        operator,
                        rhs_operand,
                    },
                    destination.into(),
                ))
            }
            s => Ok(RuleInstruction::destination(s.into())),
        }
    }
}

struct Workflow {
    rules: Vec<RuleInstruction>,
}

impl FromStr for Workflow {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Workflow {
            rules: s.split(',').map(|e| e.parse()).collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Part {
    elems: HashMap<char, usize>,
}

impl Part {
    fn sum(&self) -> usize {
        self.elems.values().sum()
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Part {
            elems: HashMap::from_iter(
                s.trim_matches('{')
                    .trim_matches('}')
                    .split(',')
                    .map(|p| p.split_once('=').unwrap())
                    .map(|(chr, val)| (chr.chars().next().unwrap(), val.parse().unwrap())),
            ),
        })
    }
}

struct Sorter {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl FromStr for Sorter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflow_lines, part_lines) = s.split_once("\n\n").ok_or(())?;
        let workflows = HashMap::from_iter(workflow_lines.lines().map(|line| {
            let (label, rest) = line.split_once('{').unwrap();
            let rules_str = &rest[..rest.len() - 1];
            (label.to_string(), rules_str.parse().unwrap())
        }));

        let parts = part_lines
            .lines()
            .map(|l| l.parse::<Part>())
            .collect::<Result<_, _>>()?;
        Ok(Sorter { workflows, parts })
    }
}

impl Sorter {
    fn execute_workflows(&self) -> usize {
        let mut accepted = vec![];
        for part in &self.parts {
            let mut workflow = self.workflows.get("in").unwrap();
            'workflow_loop: loop {
                'rule_loop: for rule in &workflow.rules {
                    let destination = if rule.condition(&part.elems) {
                        Some(&rule.destination)
                    } else {
                        None
                    };
                    match destination {
                        Some(Destination::Accept) => {
                            accepted.push(part);
                            break 'workflow_loop;
                        }
                        Some(Destination::Reject) => break 'workflow_loop,
                        Some(Destination::Workflow(new_workflow)) => {
                            workflow = self.workflows.get(new_workflow).unwrap();
                            break 'rule_loop;
                        }
                        // Move on to the next rule
                        None => (),
                    }
                }
            }
        }
        accepted.iter().map(|part| part.sum()).sum()
    }

    fn determine_distinct_combinations(&self) -> usize {
        let mut queue: VecDeque<Node> =
            VecDeque::from([Node::new(self.workflows.get("in").unwrap())]);

        let mut result: Vec<HashMap<char, RangeInclusive<usize>>> = vec![];
        while let Some(node) = queue.pop_front() {
            node.workflow
                .rules
                .iter()
                .fold_while(node.bounds.clone(), |bounds, rule| {
                    let (current_bounds, next_bounds) = match &rule.operation {
                        Some(operation) => {
                            let op_range = bounds.get(&operation.key).unwrap().clone();
                            let (current_range, next_range) = match operation.operator {
                                '>' => op_range.split_upper(operation.rhs_operand),
                                '<' => op_range.split_lower(operation.rhs_operand),
                                _ => unreachable!(),
                            };
                            let current_bounds = current_range.map(|r| {
                                let mut new_bounds = bounds.clone();
                                new_bounds.insert(operation.key, r);
                                new_bounds
                            });
                            let next_bounds = next_range.map(|r| {
                                let mut new_bounds = bounds.clone();
                                new_bounds.insert(operation.key, r);
                                new_bounds
                            });
                            (current_bounds, next_bounds)
                        }
                        None => (Some(bounds.clone()), Some(bounds.clone())),
                    };

                    match (rule.destination.clone(), current_bounds) {
                        (Destination::Accept, Some(bounds)) => result.push(bounds),
                        (Destination::Reject, _) => (),
                        (Destination::Workflow(target), Some(bounds)) => {
                            let workflow = self.workflows.get(&target).unwrap();
                            queue.push_back(Node { bounds, workflow })
                        }
                        _ => (),
                    }

                    match next_bounds {
                        Some(new_bounds) => FoldWhile::Continue(new_bounds),
                        None => FoldWhile::Done(bounds),
                    }
                });
        }

        result
            .iter()
            .map(|l| l.values().map(|x| x.len()).product::<usize>())
            .sum::<usize>()
    }
}

struct Node<'a> {
    bounds: HashMap<char, RangeInclusive<usize>>,
    workflow: &'a Workflow,
}

impl<'a> Node<'a> {
    fn new(workflow: &Workflow) -> Node {
        Node {
            bounds: HashMap::from([
                ('x', 1..=4000),
                ('m', 1..=4000),
                ('a', 1..=4000),
                ('s', 1..=4000),
            ]),
            workflow,
        }
    }
}

pub fn print_solution() {
    let sorter: Sorter = INPUT.parse().unwrap();
    println!("Sum of accepted parts: {}", sorter.execute_workflows());
    println!(
        "All possible combinations of accepted parts: {}",
        sorter.determine_distinct_combinations()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_sorting() {
        let sorter: Sorter = SAMPLE.parse().unwrap();

        assert_eq!(sorter.execute_workflows(), 19114);
    }

    #[test]
    fn test_determine_combinations() {
        let sorter: Sorter = SAMPLE.parse().unwrap();

        assert_eq!(sorter.determine_distinct_combinations(), 167409079868000);
    }
}
