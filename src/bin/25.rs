use advent_of_code::utils::{parse_input_by_lines, Parsable};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::IResult;
use rand::prelude::SliceRandom;
use std::collections::HashSet;
advent_of_code::solution!(25);

struct Node {
    name: String,
    neighbors: Vec<String>,
}

impl Parsable<'_> for Node {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, neighbors) = separated_list1(tag(" "), alpha1)(input)?;

        Ok((
            input,
            Self {
                name: name.to_string(),
                neighbors: neighbors.into_iter().map(String::from).collect(),
            },
        ))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Node>> {
    parse_input_by_lines(Node::parse)(input)
}

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
}

impl Edge {
    fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}

fn try_min_cut(edges: &[Edge]) -> Option<(usize, HashSet<String>, HashSet<String>)> {
    let nodes = edges
        .iter()
        .flat_map(|edge| vec![&edge.from, &edge.to])
        .unique()
        .collect::<Vec<_>>();

    let mut node_count = nodes.len();
    let mut current_edges = edges.to_vec();

    let mut chosen = vec![];
    let mut rng = rand::thread_rng();

    while node_count != 2 {
        let edge = current_edges.choose(&mut rng).unwrap();
        chosen.push(edge.clone());

        let convert = |name: &str| {
            if name == edge.to {
                edge.from.clone()
            } else {
                name.to_string()
            }
        };

        current_edges = current_edges
            .iter()
            .map(|e| Edge::new(convert(&e.from), convert(&e.to)))
            .filter(|e| e.from != e.to)
            .collect::<Vec<_>>();
        node_count -= 1;
    }

    let [a, b] = current_edges
        .iter()
        .flat_map(|edge| vec![&edge.from, &edge.to])
        .unique()
        .collect::<Vec<_>>()[..]
    else {
        unreachable!("Didn't end up with 2 nodes in graph");
    };

    let mut left = HashSet::from([a.to_string()]);
    let mut right = HashSet::from([b.to_string()]);

    for edge in chosen.iter().rev() {
        if left.contains(&edge.from) {
            left.insert(edge.to.to_string());
        } else if right.contains(&edge.from) {
            right.insert(edge.to.to_string());
        } else {
            unreachable!("Node not in either side of the cut")
        }
    }

    println!("Current edges: {:?}", current_edges);
    println!("Left size: {:?}, Right size: {:?}", left.len(), right.len());

    Some((current_edges.len(), left, right))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, nodes) = parse(input).unwrap();
    let edges = nodes
        .iter()
        .flat_map(|node| {
            node.neighbors
                .iter()
                .map(move |neighbor| Edge::new(node.name.clone(), neighbor.clone()))
        })
        .collect::<Vec<_>>();

    loop {
        let Some((result, left, right)) = try_min_cut(&edges) else {
            return None;
        };

        println!("Result: {}", result);

        if result == 3 {
            return Some(left.len() * right.len());
        }
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
