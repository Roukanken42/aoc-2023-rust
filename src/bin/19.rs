use std::collections::HashMap;
use std::ops::{Index, IndexMut, Range};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{map, value};
use nom::multi::{count, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;

use advent_of_code::utils::{parse_input, Parsable};

advent_of_code::solution!(19);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WorkflowType {
    Accepted,
    Rejected,
    Custom(String),
}

impl Parsable<'_> for WorkflowType {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(WorkflowType::Accepted, tag("A")),
            value(WorkflowType::Rejected, tag("R")),
            map(many1(one_of("abcdefghijklmnopqrstuvwxyz")), |s| {
                WorkflowType::Custom(s.into_iter().collect())
            }),
        ))(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Attribute {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Parsable<'_> for Attribute {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Attribute::ExtremelyCool, tag("x")),
            value(Attribute::Musical, tag("m")),
            value(Attribute::Aerodynamic, tag("a")),
            value(Attribute::Shiny, tag("s")),
        ))(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Condition {
    LessThan(u32),
    MoreThan(u32),
}

impl Condition {
    fn is_satisfied_by(&self, value: u32) -> bool {
        match self {
            Condition::LessThan(x) => value < *x,
            Condition::MoreThan(x) => value > *x,
        }
    }

    fn split_range(&self, range: &Range<u32>) -> (Range<u32>, Range<u32>) {
        match self {
            Condition::LessThan(x) => (range.start..*x, *x..range.end),
            Condition::MoreThan(x) => ((*x + 1)..range.end, range.start..(*x + 1)),
        }
    }
}

impl Parsable<'_> for Condition {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(preceded(tag("<"), u32::parse), Condition::LessThan),
            map(preceded(tag(">"), u32::parse), Condition::MoreThan),
        ))(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    Conditional {
        attribute: Attribute,
        condition: Condition,
        next: WorkflowType,
    },
    Fallback {
        next: WorkflowType,
    },
}

impl Parsable<'_> for Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                tuple((
                    Attribute::parse,
                    Condition::parse,
                    tag(":"),
                    WorkflowType::parse,
                )),
                |(attribute, condition, _, next)| Rule::Conditional {
                    attribute,
                    condition,
                    next,
                },
            ),
            map(WorkflowType::parse, |next| Rule::Fallback { next }),
        ))(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    workflow_type: WorkflowType,
    rules: Vec<Rule>,
}

impl Workflow {
    fn execute(&self, part: &Part) -> Option<WorkflowType> {
        for rule in self.rules.iter() {
            match rule {
                Rule::Conditional {
                    attribute,
                    condition,
                    next,
                } => {
                    if condition.is_satisfied_by(part[*attribute]) {
                        return Some(next.clone());
                    }
                }
                Rule::Fallback { next } => {
                    return Some(next.clone());
                }
            }
        }

        None
    }

    fn execute_range(&self, part: &PartRange) -> Vec<(WorkflowType, PartRange)> {
        let mut result = vec![];
        let mut current = part.clone();

        for rule in self.rules.iter() {
            match rule {
                Rule::Conditional {
                    attribute,
                    condition,
                    next,
                } => {
                    let (matched, unmatched) = condition.split_range(&current[*attribute]);
                    if !matched.is_empty() {
                        let mut next_part = current.clone();
                        next_part[*attribute] = matched;
                        result.push((next.clone(), next_part));
                    }

                    if unmatched.is_empty() {
                        break;
                    }

                    current[*attribute] = unmatched;
                }
                Rule::Fallback { next } => {
                    result.push((next.clone(), current.clone()));
                    break;
                }
            }
        }

        result
    }
}

impl Parsable<'_> for Workflow {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, workflow_type) = WorkflowType::parse(input)?;
        let (input, rules) =
            delimited(tag("{"), separated_list0(tag(","), Rule::parse), tag("}"))(input)?;

        Ok((
            input,
            Self {
                workflow_type,
                rules,
            },
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Part {
    coolness: u32,
    musicality: u32,
    aerodynamicity: u32,
    shininess: u32,
}

impl Part {
    fn new() -> Self {
        Self {
            coolness: 0,
            musicality: 0,
            aerodynamicity: 0,
            shininess: 0,
        }
    }

    fn attribute_sum(&self) -> u32 {
        self.coolness + self.musicality + self.aerodynamicity + self.shininess
    }
}

impl Index<Attribute> for Part {
    type Output = u32;

    fn index(&self, index: Attribute) -> &Self::Output {
        match index {
            Attribute::ExtremelyCool => &self.coolness,
            Attribute::Musical => &self.musicality,
            Attribute::Aerodynamic => &self.aerodynamicity,
            Attribute::Shiny => &self.shininess,
        }
    }
}

impl IndexMut<Attribute> for Part {
    fn index_mut(&mut self, index: Attribute) -> &mut Self::Output {
        match index {
            Attribute::ExtremelyCool => &mut self.coolness,
            Attribute::Musical => &mut self.musicality,
            Attribute::Aerodynamic => &mut self.aerodynamicity,
            Attribute::Shiny => &mut self.shininess,
        }
    }
}

impl Parsable<'_> for Part {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, attributes) = delimited(
            tag("{"),
            separated_list0(
                tag(","),
                separated_pair(Attribute::parse, tag("="), u32::parse),
            ),
            tag("}"),
        )(input)?;

        let mut part = Self::new();
        for (attribute, value) in attributes {
            part[attribute] = value;
        }

        Ok((input, part))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PartRange {
    coolness: Range<u32>,
    musicality: Range<u32>,
    aerodynamicity: Range<u32>,
    shininess: Range<u32>,
}

impl PartRange {
    fn new() -> Self {
        Self {
            coolness: 1..4001,
            musicality: 1..4001,
            aerodynamicity: 1..4001,
            shininess: 1..4001,
        }
    }

    fn combinations_count(&self) -> u64 {
        self.coolness.len() as u64
            * self.musicality.len() as u64
            * self.aerodynamicity.len() as u64
            * self.shininess.len() as u64
    }
}

impl Index<Attribute> for PartRange {
    type Output = Range<u32>;

    fn index(&self, index: Attribute) -> &Self::Output {
        match index {
            Attribute::ExtremelyCool => &self.coolness,
            Attribute::Musical => &self.musicality,
            Attribute::Aerodynamic => &self.aerodynamicity,
            Attribute::Shiny => &self.shininess,
        }
    }
}

impl IndexMut<Attribute> for PartRange {
    fn index_mut(&mut self, index: Attribute) -> &mut Self::Output {
        match index {
            Attribute::ExtremelyCool => &mut self.coolness,
            Attribute::Musical => &mut self.musicality,
            Attribute::Aerodynamic => &mut self.aerodynamicity,
            Attribute::Shiny => &mut self.shininess,
        }
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
    let workflows = separated_list1(line_ending, Workflow::parse);
    let sep = count(line_ending, 2);
    let parts = separated_list1(line_ending, Part::parse);

    parse_input(separated_pair(workflows, sep, parts))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (workflows, parts)) = parse(input).unwrap();

    let workflows_by_type = workflows
        .into_iter()
        .chain(vec![
            Workflow {
                workflow_type: WorkflowType::Accepted,
                rules: vec![],
            },
            Workflow {
                workflow_type: WorkflowType::Rejected,
                rules: vec![],
            },
        ])
        .map(|workflow| (workflow.workflow_type.clone(), workflow))
        .collect::<HashMap<_, _>>();

    let mut result = workflows_by_type
        .keys()
        .cloned()
        .map(|key| (key, vec![]))
        .collect::<HashMap<WorkflowType, Vec<Part>>>();

    for part in parts {
        let mut current_workflow = WorkflowType::Custom("in".to_string());

        loop {
            let Some(next) = workflows_by_type[&current_workflow].execute(&part) else {
                break;
            };

            current_workflow = next;
        }

        result
            .get_mut(&current_workflow)
            .unwrap()
            .push(part.clone());
    }

    let unprocessed = result
        .iter()
        .filter(|(workflow, parts)| {
            matches!(workflow, WorkflowType::Custom(_)) && !parts.is_empty()
        })
        .collect::<HashMap<_, _>>();

    assert_eq!(unprocessed, HashMap::new());

    result[&WorkflowType::Accepted]
        .iter()
        .map(|part| part.attribute_sum())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (workflows, _)) = parse(input).unwrap();

    let workflows_by_type = workflows
        .into_iter()
        .map(|workflow| (workflow.workflow_type.clone(), workflow))
        .collect::<HashMap<_, _>>();

    let mut queue = vec![(WorkflowType::Custom("in".to_string()), PartRange::new())];
    let mut accepted_ranges = vec![];

    while let Some((workflow_type, part_range)) = queue.pop() {
        match workflow_type {
            WorkflowType::Accepted => {
                println!(
                    "accepted {:?} -> {}",
                    part_range,
                    part_range.combinations_count()
                );
                accepted_ranges.push(part_range);
            }
            WorkflowType::Rejected => {
                println!(
                    "rejected {:?} -> -{}",
                    part_range,
                    part_range.combinations_count()
                );
            }
            WorkflowType::Custom(_) => {
                let vec1 = workflows_by_type[&workflow_type].execute_range(&part_range);
                println!("[{:?}] {:?} -> {:?}", workflow_type, part_range, vec1);
                queue.extend(vec1)
            }
        }
    }

    println!();
    accepted_ranges
        .iter()
        .map(|part_range| {
            let x = part_range.combinations_count();
            println!("{:?} -> {}", part_range, x);
            x
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow_type() {
        assert_eq!(WorkflowType::parse("A"), Ok(("", WorkflowType::Accepted)));
        assert_eq!(WorkflowType::parse("R"), Ok(("", WorkflowType::Rejected)));
        assert_eq!(
            WorkflowType::parse("abc"),
            Ok(("", WorkflowType::Custom("abc".to_string())))
        );
    }

    #[test]
    fn test_parse_attribute() {
        assert_eq!(Attribute::parse("x"), Ok(("", Attribute::ExtremelyCool)));
        assert_eq!(Attribute::parse("m"), Ok(("", Attribute::Musical)));
        assert_eq!(Attribute::parse("a"), Ok(("", Attribute::Aerodynamic)));
        assert_eq!(Attribute::parse("s"), Ok(("", Attribute::Shiny)));
    }

    #[test]
    fn test_parse_condition() {
        assert_eq!(Condition::parse("<42"), Ok(("", Condition::LessThan(42))));
        assert_eq!(Condition::parse(">1"), Ok(("", Condition::MoreThan(1))));
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(
            Rule::parse("x<42:A"),
            Ok((
                "",
                Rule::Conditional {
                    attribute: Attribute::ExtremelyCool,
                    condition: Condition::LessThan(42),
                    next: WorkflowType::Accepted,
                }
            ))
        );
        assert_eq!(
            Rule::parse("x<42:R"),
            Ok((
                "",
                Rule::Conditional {
                    attribute: Attribute::ExtremelyCool,
                    condition: Condition::LessThan(42),
                    next: WorkflowType::Rejected,
                }
            ))
        );
        assert_eq!(
            Rule::parse("x<42:abc"),
            Ok((
                "",
                Rule::Conditional {
                    attribute: Attribute::ExtremelyCool,
                    condition: Condition::LessThan(42),
                    next: WorkflowType::Custom("abc".to_string()),
                }
            ))
        );
        assert_eq!(
            Rule::parse("A"),
            Ok((
                "",
                Rule::Fallback {
                    next: WorkflowType::Accepted,
                }
            ))
        );
        assert_eq!(
            Rule::parse("R"),
            Ok((
                "",
                Rule::Fallback {
                    next: WorkflowType::Rejected,
                }
            ))
        );
        assert_eq!(
            Rule::parse("abc"),
            Ok((
                "",
                Rule::Fallback {
                    next: WorkflowType::Custom("abc".to_string()),
                }
            ))
        );
    }

    #[test]
    fn test_parse_workflow() {
        assert_eq!(
            Workflow::parse("A{}"),
            Ok((
                "",
                Workflow {
                    workflow_type: WorkflowType::Accepted,
                    rules: vec![]
                }
            ))
        );
        assert_eq!(
            Workflow::parse("A{a<206:qjq,R}"),
            Ok((
                "",
                Workflow {
                    workflow_type: WorkflowType::Accepted,
                    rules: vec![
                        Rule::Conditional {
                            attribute: Attribute::Aerodynamic,
                            condition: Condition::LessThan(206),
                            next: WorkflowType::Custom("qjq".to_string())
                        },
                        Rule::Fallback {
                            next: WorkflowType::Rejected
                        }
                    ]
                }
            ))
        );
        assert_eq!(
            Workflow::parse("xkc{x<42:A,x>42:R}"),
            Ok((
                "",
                Workflow {
                    workflow_type: WorkflowType::Custom("xkc".to_string()),
                    rules: vec![
                        Rule::Conditional {
                            attribute: Attribute::ExtremelyCool,
                            condition: Condition::LessThan(42),
                            next: WorkflowType::Accepted
                        },
                        Rule::Conditional {
                            attribute: Attribute::ExtremelyCool,
                            condition: Condition::MoreThan(42),
                            next: WorkflowType::Rejected
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_parse_part() {
        assert_eq!(
            Part::parse("{x=1,m=2,a=3,s=4}"),
            Ok((
                "",
                Part {
                    coolness: 1,
                    musicality: 2,
                    aerodynamicity: 3,
                    shininess: 4
                }
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
