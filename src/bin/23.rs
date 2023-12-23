use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use advent_of_code::utils::location::{direction, Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(23);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Location<i32>),
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Path, tag(".")),
            value(Self::Forest, tag("#")),
            value(Self::Slope(direction::UP), tag("^")),
            value(Self::Slope(direction::RIGHT), tag(">")),
            value(Self::Slope(direction::DOWN), tag("v")),
            value(Self::Slope(direction::LEFT), tag("<")),
        ))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    parse_input_by_lines(many1(Tile::parse))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirectedEdge {
    From(usize),
    To(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    size: usize,
    edges: Vec<DirectedEdge>,
}

impl Node {
    fn edges_from(&self) -> impl Iterator<Item = usize> + '_ {
        self.edges.iter().filter_map(|edge| match edge {
            DirectedEdge::From(from) => Some(*from),
            _ => None,
        })
    }

    fn edges_to(&self) -> impl Iterator<Item = usize> + '_ {
        self.edges.iter().filter_map(|edge| match edge {
            DirectedEdge::To(to) => Some(*to),
            _ => None,
        })
    }

    fn all_edges(&self) -> impl Iterator<Item = usize> + '_ {
        self.edges.iter().map(|edge| match edge {
            DirectedEdge::From(from) => *from,
            DirectedEdge::To(to) => *to,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MarkedTile {
    Mark(usize),
    Tile(Tile),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Graph {
    nodes: Vec<Node>,
    start: usize,
    end: usize,
}

fn construct_graph(map: &Vec<Vec<Tile>>) -> Graph {
    let mut marked_map = map
        .iter()
        .map(|row| row.iter().map(|tile| MarkedTile::Tile(*tile)).collect_vec())
        .collect_vec();

    let mut current_mark = 0;
    let mut graph = vec![Node { size: 0, edges: vec![] }];

    for loc in marked_map.iter_2d_keys() {
        let loc = loc.map(|x| x as i32);
        let Some(tile) = marked_map.get_2d(loc) else {
            continue;
        };

        match tile {
            MarkedTile::Mark(_) => continue,
            MarkedTile::Tile(Tile::Forest) => continue,
            MarkedTile::Tile(Tile::Path) => {
                let mut queue = vec![loc];
                let mut size = 0;

                while let Some(loc) = queue.pop() {
                    match marked_map.get_2d(loc) {
                        Some(MarkedTile::Tile(Tile::Path)) => {
                            marked_map.set_2d(loc, MarkedTile::Mark(current_mark));
                            size += 1;

                            let walkable_neighbors = loc
                                .iter_adjacent()
                                .into_iter()
                                .filter(|&loc| {
                                    marked_map
                                        .get_2d(loc)
                                        .map(|tile| matches!(tile, MarkedTile::Mark(_) | MarkedTile::Tile(Tile::Path)))
                                        .unwrap_or(false)
                                })
                                .count();

                            assert!(walkable_neighbors <= 2, "Input has some unclear path");

                            queue.extend(loc.iter_adjacent());
                        }
                        Some(MarkedTile::Tile(Tile::Slope(direction))) => {
                            let from = loc - *direction;
                            let to = loc + *direction;

                            let Some(MarkedTile::Mark(from_mark)) = marked_map.get_2d(from) else {
                                continue;
                            };
                            let Some(MarkedTile::Mark(to_mark)) = marked_map.get_2d(to) else {
                                continue;
                            };

                            // println!("{} -> {};", from_mark, to_mark);

                            graph[*from_mark].edges.push(DirectedEdge::To(*to_mark));
                            graph[*to_mark].edges.push(DirectedEdge::From(*from_mark));
                        }
                        _ => {}
                    };
                }

                graph[current_mark].size = size;
                current_mark += 1;
                graph.push(Node { size: 0, edges: vec![] })
            }
            _ => continue,
        }
    }

    graph.pop();

    let unhandled_nodes = graph.iter().filter(|node| node.edges.len() > 2 && node.size > 1).count();
    assert_eq!(unhandled_nodes, 0, "Input has some unhandled nodes");

    let start = marked_map[0]
        .iter()
        .filter_map(|tile| if let MarkedTile::Mark(mark) = tile { Some(mark) } else { None })
        .next()
        .unwrap();

    let end = marked_map
        .last()
        .unwrap()
        .iter()
        .filter_map(|tile| if let MarkedTile::Mark(mark) = tile { Some(mark) } else { None })
        .next()
        .unwrap();

    Graph {
        nodes: graph,
        start: *start,
        end: *end,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();
    let graph = construct_graph(&map);

    let mut longest_paths = graph.nodes.iter().map(|_| None).collect_vec();
    let mut queue = vec![graph.end];

    while let Some(current) = queue.pop() {
        let node = &graph.nodes[current];

        let distances_to_end = node.edges_to().map(|to| longest_paths[to]).collect_vec();
        if distances_to_end.iter().any(|distance| distance.is_none()) {
            continue;
        }

        let max_distance = distances_to_end.iter().map(|a| a.unwrap()).max().unwrap_or(0);

        longest_paths[current] = Some(max_distance + node.size + 1);
        queue.extend(node.edges_from());
    }

    longest_paths[graph.start].map(|x| x - 2)
}

fn recursive_find_longest(graph: &Graph, current: usize, visited: &mut Vec<bool>) -> Option<usize> {
    let node = &graph.nodes[current];

    if current == graph.end {
        return Some(node.size);
    }

    let mut max_distance = None;

    visited[current] = true;
    for to in node.all_edges() {
        if visited[to] {
            continue;
        }

        let recursive = recursive_find_longest(graph, to, visited);
        // println!("{} -> {} = {:?}", current, to, recursive);

        max_distance = [max_distance, recursive].into_iter().flatten().reduce(usize::max);
    }
    visited[current] = false;

    max_distance.map(|max| max + node.size + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();
    let graph = construct_graph(&map);

    recursive_find_longest(&graph, graph.start, &mut vec![false; graph.nodes.len()]).map(|x| x - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
