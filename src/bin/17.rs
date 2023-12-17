use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::ops::Add;
use std::str::FromStr;

use nom::character::complete::one_of;
use nom::combinator::{map_res, recognize};
use nom::multi::many1;
use nom::IResult;
use num::Zero;

use advent_of_code::utils::location::{direction, Access2d, Location};
use advent_of_code::utils::parse_input_by_lines;

advent_of_code::solution!(17);

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    parse_input_by_lines(many1(map_res(
        recognize(one_of("0123456789")),
        i32::from_str,
    )))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Crucible {
    location: Location<i32>,
    direction: Location<i32>,
    straight_count: u32,
}

impl Crucible {
    fn go_straight(&self) -> Option<Self> {
        if self.straight_count == 3 {
            None
        } else {
            Some(Self {
                location: self.location + self.direction,
                direction: self.direction,
                straight_count: self.straight_count + 1,
            })
        }
    }

    fn go_right(&self) -> Self {
        let direction_right = self.direction.rotate_90_cw();
        Self {
            location: self.location + direction_right,
            direction: direction_right,
            straight_count: 1,
        }
    }

    fn go_left(&self) -> Self {
        let direction_left = self.direction.rotate_90_ccw();
        Self {
            location: self.location + direction_left,
            direction: direction_left,
            straight_count: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct UltraCrucible {
    location: Location<i32>,
    direction: Location<i32>,
    straight_count: u32,
}

impl UltraCrucible {
    fn go_straight(&self) -> Option<Self> {
        if self.straight_count == 10 {
            None
        } else {
            Some(Self {
                location: self.location + self.direction,
                direction: self.direction,
                straight_count: self.straight_count + 1,
            })
        }
    }

    fn go_right(&self) -> Option<Self> {
        if self.straight_count < 4 {
            return None;
        }

        let direction_right = self.direction.rotate_90_cw();
        Some(Self {
            location: self.location + direction_right,
            direction: direction_right,
            straight_count: 1,
        })
    }

    fn go_left(&self) -> Option<Self> {
        if self.straight_count < 4 {
            return None;
        }

        let direction_left = self.direction.rotate_90_ccw();
        Some(Self {
            location: self.location + direction_left,
            direction: direction_left,
            straight_count: 1,
        })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, data) = parse(input).unwrap();

    let start = Location::new(0, 0);
    let target = Location::new(data[0].len() as i32 - 1, data.len() as i32 - 1);

    let mut priority = BinaryHeap::new();
    priority.push(Reverse((
        0,
        0,
        Crucible {
            location: start,
            direction: direction::RIGHT,
            straight_count: 0,
        },
    )));

    let mut visited = HashSet::new();

    while let Some(Reverse((_, distance, state))) = priority.pop() {
        if !visited.insert(state) {
            continue;
        }

        let cost = match (state.location, data.get_2d(state.location)) {
            (loc, _) if loc == start => 0,
            (_, Some(x)) => *x,
            (_, None) => continue,
        };

        if state.location == target {
            return Some(distance + cost);
        }

        if let Some(next) = state.go_straight() {
            priority.push(Reverse((
                distance + cost + next.location.manhattan_distance(target),
                distance + cost,
                next,
            )));
        }

        let left = state.go_left();
        priority.push(Reverse((
            distance + cost + left.location.manhattan_distance(target),
            distance + cost,
            left,
        )));

        let right = state.go_right();
        priority.push(Reverse((
            distance + cost + right.location.manhattan_distance(target),
            distance + cost,
            right,
        )));
    }

    None
}

fn a_star<
    State: Ord + Hash + Eq + Copy,
    Value: Ord + Copy + Add<Output = Value> + Zero,
    NextIter: IntoIterator<Item = (State, Value)>,
>(
    starting_states: &[State],
    is_target: impl Fn(&State) -> bool,
    heuristics: impl Fn(&State) -> Value,
    next_states: impl Fn(&State) -> NextIter,
) -> Option<Value> {
    let mut priority = BinaryHeap::from_iter(
        starting_states
            .into_iter()
            .filter_map(|state| Some(Reverse((heuristics(state), Value::zero(), *state)))),
    );

    let mut visited = HashSet::new();

    while let Some(Reverse((_, cost, state))) = priority.pop() {
        if !visited.insert(state) {
            continue;
        }

        if is_target(&state) {
            return Some(cost);
        }

        for (next_state, next_cost) in next_states(&state) {
            let next_total_cost = cost + next_cost;

            priority.push(Reverse((
                next_total_cost + heuristics(&next_state),
                next_total_cost,
                next_state,
            )));
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, data) = parse(input).unwrap();

    let start = Location::new(0, 0);
    let target = Location::new(data[0].len() as i32 - 1, data.len() as i32 - 1);

    let starting_states = vec![
        UltraCrucible {
            location: start,
            direction: direction::RIGHT,
            straight_count: 0,
        },
        UltraCrucible {
            location: start,
            direction: direction::DOWN,
            straight_count: 0,
        },
    ];

    let mut distances = vec![vec![0; data[0].len()]; data.len()];
    let mut queue = BinaryHeap::from([Reverse((0, target))]);

    while let Some(Reverse((distance, now))) = queue.pop() {
        let Some(&now_cost) = data.get_2d(now) else {
            continue;
        };

        if distances.get_2d(now).copied().unwrap_or(0) != 0 {
            continue;
        }
        distances.set_2d(now, distance);

        for next in now.iter_adjacent() {
            queue.push(Reverse((distance + now_cost, next)));
        }
    }

    a_star(
        &starting_states,
        |state| state.location == target && state.straight_count >= 4,
        |state| distances.get_2d(state.location).copied().unwrap_or(0),
        |state| {
            [state.go_straight(), state.go_left(), state.go_right()]
                .into_iter()
                .filter_map(|next| Some((next?, data.get_2d(next?.location).copied()?)))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
