use std::ops::{Add, Sub};

use common::*;

#[derive(Debug, PartialEq, Default, Clone, Copy, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

fn normalize_int(x: i32) -> i32 {
    match x {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    }
}

fn grid_normalize(point: Point) -> Point {
    if point.x.abs() == point.y.abs() {
        Point {
            x: normalize_int(point.x),
            y: normalize_int(point.y),
        }
    } else {
        match point {
            p if p.y > 0 && p.y > p.x && p.y > -p.x => Point { x: 0, y: 1 },
            p if p.x > 0 && p.x > p.y && p.y > -p.x => Point { x: 1, y: 0 },
            p if p.y < 0 && p.y < p.x && p.y < -p.x => Point { x: 0, y: -1 },
            p if p.x < 0 && p.x < p.y && p.y < -p.x => Point { x: -1, y: 0 },
            _ => Point { x: 0, y: 0 },
        }
    }
}

enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Heading {
    type Error = Failure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let heading = match value {
            "U" => Heading::Up,
            "D" => Heading::Down,
            "L" => Heading::Left,
            "R" => Heading::Right,
            _ => return Err(e!(r#"Invalid &str for a Heading: "{value}""#)),
        };

        Ok(heading)
    }
}

struct Direction {
    heading: Heading,
    steps: u32,
}

impl TryFrom<&str> for Direction {
    type Error = Failure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [heading, steps] = value.split(' ').collect::<Vec<_>>()[..] else {
            return Err(e!("Tried to get a Direction from an invalid string: {value}"));
        };

        let heading = Heading::try_from(heading)?;
        let steps = steps
            .parse()
            .map_err(|err| e!("Failed to parse Direction.steps string: {err}"))?;

        Ok(Self { heading, steps })
    }
}

fn main() -> Result<()> {
    let data = get_input()?;
    let mut rope = vec![Point::default(); 2];
    let mut tail_visited = vec![];

    for line in data.split('\n') {
        let direction = Direction::try_from(line)?;
        for _ in 0..direction.steps {
            let mut new_rope = vec![];
            let mut cursor = *rope.first().expect("`rope` must have at least one element");
            match direction.heading {
                Heading::Up => cursor.y += 1,
                Heading::Down => cursor.y -= 1,
                Heading::Left => cursor.x -= 1,
                Heading::Right => cursor.x += 1,
            }
            new_rope.push(cursor);

            for segment in rope.iter().skip(1) {
                let diff = cursor - *segment;
                let diff = grid_normalize(diff);
                new_rope.push(cursor - diff);
            }
            rope = new_rope;
            tail_visited.push(*rope.last().expect("`rope` must have at least one element"));
        }
    }

    tail_visited.sort();
    tail_visited.dedup();

    println!("{:?}", tail_visited.len());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn grid_normalize() {
        let new_tail = super::grid_normalize(Point { x: 0, y: 0 });
        assert_eq!(new_tail, Point { x: 0, y: 0 });

        let new_tail = super::grid_normalize(Point { x: 1, y: 1 });
        assert_eq!(new_tail, Point { x: 1, y: 1 });

        let new_tail = super::grid_normalize(Point { x: 2, y: 2 });
        assert_eq!(new_tail, Point { x: 1, y: 1 });

        let new_tail = super::grid_normalize(Point { x: -2, y: -2 });
        assert_eq!(new_tail, Point { x: -1, y: -1 });

        let new_tail = super::grid_normalize(Point { x: 2, y: 1 });
        assert_eq!(new_tail, Point { x: 1, y: 0 });

        let new_tail = super::grid_normalize(Point { x: 1, y: 2 });
        assert_eq!(new_tail, Point { x: 0, y: 1 });

        let new_tail = super::grid_normalize(Point { x: -2, y: 1 });
        assert_eq!(new_tail, Point { x: -1, y: 0 });

        let new_tail = super::grid_normalize(Point { x: 1, y: -2 });
        assert_eq!(new_tail, Point { x: 0, y: -1 });
    }
}
