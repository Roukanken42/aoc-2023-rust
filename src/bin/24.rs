use std::ops::RangeBounds;

use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

use advent_of_code::utils::location3d::Location3d;
use advent_of_code::utils::{parse_input_by_lines, Parsable};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    position: Location3d<f64>,
    velocity: Location3d<f64>,
}

impl Parsable<'_> for Hailstone {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_num = |input| preceded(space0, i64::parse)(input);
        let parse_coords = |input| tuple((parse_num, tag(", "), parse_num, tag(", "), parse_num))(input);

        let (input, (coords1, coords2)) = separated_pair(parse_coords, tag(" @ "), parse_coords)(input)?;
        let (x, _, y, _, z) = coords1;
        let (vx, _, vy, _, vz) = coords2;

        Ok((
            input,
            Self {
                position: Location3d::new(x, y, z).map(|a| a as f64),
                velocity: Location3d::new(vx, vy, vz).map(|a| a as f64),
            },
        ))
    }
}

impl Hailstone {
    fn adjust_velocity(&self, velocity: Location3d<f64>) -> Self {
        Self {
            position: self.position,
            velocity: self.velocity - velocity,
        }
    }
}

fn intersect(hailstone1: &Hailstone, hailstone2: &Hailstone) -> Option<Location3d<f64>> {
    let diff = hailstone2.position - hailstone1.position;
    let intersection_time_2 = (hailstone1.velocity.y * diff.x - hailstone1.velocity.x * diff.y)
        / (hailstone1.velocity.x * hailstone2.velocity.y - hailstone1.velocity.y * hailstone2.velocity.x);

    let intersection_time_1 = (diff.x + hailstone2.velocity.x * intersection_time_2) / hailstone1.velocity.x;
    let intersection = hailstone2.position + hailstone2.velocity * intersection_time_2;

    if intersection_time_1 < 0.0 || intersection_time_2 < 0.0 {
        return None;
    }
    Some(intersection)
}

fn find_intersections_2d(hailstones: &[Hailstone], is_valid: impl Fn(&Location3d<f64>) -> bool) -> usize {
    let mut intersection_count = 0usize;

    for (i, hailstone1) in hailstones.iter().enumerate() {
        for hailstone2 in hailstones.iter().skip(i + 1) {
            let Some(intersection) = intersect(hailstone1, hailstone2) else {
                continue;
            };

            if is_valid(&intersection) {
                intersection_count += 1;
            }
        }
    }

    intersection_count
}

fn parse(input: &str) -> IResult<&str, Vec<Hailstone>> {
    parse_input_by_lines(Hailstone::parse)(input)
}

fn is_in_square_2d(range: impl RangeBounds<f64>) -> impl Fn(&Location3d<f64>) -> bool {
    move |pos| range.contains(&pos.x) && range.contains(&pos.y)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, hailstones) = parse(input).unwrap();

    Some(find_intersections_2d(
        &hailstones,
        is_in_square_2d(200_000_000_000_000.0..=400_000_000_000_000.0),
    ))
}

fn find_one_intersection_point_2d(hailstones: impl Iterator<Item = Hailstone>) -> Option<Location3d<f64>> {
    let mut intersection_point = None;
    let mut iter = hailstones.into_iter();
    let check_stone = iter.next()?;

    for hailstone in iter {
        let Some(intersection) = intersect(&check_stone, &hailstone) else {
            return None;
        };

        match intersection_point {
            None => intersection_point = Some(intersection),
            Some(point) if point.to_2d() != intersection.to_2d() => return None,
            _ => {}
        }
    }

    intersection_point
}

fn find_one_intersection_point_2d_with_velocity(
    hailstones: &[Hailstone],
    velocity: Location3d<f64>,
) -> Option<Location3d<f64>> {
    find_one_intersection_point_2d(hailstones.iter().map(|hailstone| hailstone.adjust_velocity(velocity)))
}

fn find_vz_from_xy_velocity_and_intersection(
    hailstones: &[Hailstone],
    intersection: &Location3d<f64>,
    velocity: &Location3d<f64>,
) -> Option<f64> {
    let mut vz = None;
    let mut iter = hailstones.iter().map(|hailstone| hailstone.adjust_velocity(*velocity));

    let check_stone = iter.next()?;
    let check_time = get_time_2d(&check_stone, intersection);
    let check_z = check_stone.position.z + check_time * check_stone.velocity.z;

    for hailstone in iter {
        let time = get_time_2d(&hailstone, intersection);
        let z = hailstone.position.z + time * hailstone.velocity.z;

        if time == check_time {
            continue;
        }

        let new_vz = (z - check_z) / (time - check_time);
        match vz {
            None => vz = Some(new_vz),
            Some(old_vz) if old_vz != new_vz => return None,
            _ => {}
        }
    }

    vz
}

fn get_time_2d(hailstone: &Hailstone, intersection: &Location3d<f64>) -> f64 {
    if hailstone.velocity.x == 0.0 {
        return (intersection.y - hailstone.position.y) / hailstone.velocity.y;
    }
    (intersection.x - hailstone.position.x) / hailstone.velocity.x
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, hailstones) = parse(input).unwrap();

    for x in 0..=500 {
        for y in 0..=500 {
            for velocity in [
                Location3d::new(x, y, 0),
                Location3d::new(-x, y, 0),
                Location3d::new(x, -y, 0),
                Location3d::new(-x, -y, 0),
            ]
            .iter()
            {
                let velocity = velocity.map(|a| a as f64);
                let Some(intersection) = find_one_intersection_point_2d_with_velocity(&hailstones, velocity) else {
                    continue;
                };

                let Some(vz) = find_vz_from_xy_velocity_and_intersection(&hailstones, &intersection, &velocity) else {
                    continue;
                };

                // println!("Found vz {}", vz);
                let z = hailstones[0].position.z
                    + get_time_2d(&hailstones[0].adjust_velocity(velocity), &intersection) * (hailstones[0].velocity.z - vz);

                return Some(intersection.x as i64 + intersection.y as i64 + z as i64);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (_, data) = parse(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(find_intersections_2d(&data, is_in_square_2d(7.0..=27.0)), 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
