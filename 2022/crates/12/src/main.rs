use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    ops::Neg,
};

use common::*;

type Array<T> = Box<[T]>;

enum Square {
    Elevation(u8),
    Start,
    End,
}

impl Square {
    fn elevation(&self) -> u8 {
        match self {
            Square::Elevation(elevation) => *elevation,
            Square::Start => b'a' - 96,
            Square::End => b'z' - 96,
        }
    }
}

type Coord = (usize, usize);

struct Matrix<T> {
    grid: Array<Array<T>>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    fn get(&self, (x, y): Coord) -> Result<&T> {
        self.grid
            .get(y)
            .ok_or_else(|| e!("Tried to get a Square outside the Y-Axis of the map"))?
            .get(x)
            .ok_or_else(|| e!("Tried to get a Square outside the X-Axis of the map"))
    }

    fn get_mut(&mut self, (x, y): Coord) -> Result<&mut T> {
        self.grid
            .get_mut(y)
            .ok_or_else(|| e!("Tried to get_mut a Square outside the Y-Axis of the map"))?
            .get_mut(x)
            .ok_or_else(|| e!("Tried to get_mut a Square outside the X-Axis of the map"))
    }
}

impl<T: Clone> Matrix<T> {
    fn from_elem(elem: T, width: usize, height: usize) -> Self {
        let vec2d = std::vec::from_elem(std::vec::from_elem(elem, width), height);
        let grid = vec2d
            .into_iter()
            .map(|v| v.into_boxed_slice())
            .collect::<Array<_>>();
        Self {
            grid,
            width,
            height,
        }
    }
}

impl<T> TryFrom<Array<Array<T>>> for Matrix<T> {
    type Error = Failure;

    fn try_from(grid: Array<Array<T>>) -> Result<Self, Self::Error> {
        let width = grid
            .get(0)
            .ok_or_else(|| e!("Matrix doesn't have any rows"))?
            .len();

        if grid.iter().skip(1).any(|row| row.len() != width) {
            return Err(e!("Matrix rows are not of the same length"));
        }

        Ok(Self {
            width,
            height: grid.len(),
            grid,
        })
    }
}

impl Debug for Matrix<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", if *cell { "▓" } else { "░" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Map {
    matrix: Matrix<Square>,
    start: Coord,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.matrix.grid.iter() {
            for cell in row.iter() {
                let ch = match cell {
                    Square::Elevation(elevation) => (elevation + 96) as char,
                    Square::Start => 'S',
                    Square::End => 'E',
                };
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    fn invert(self) -> Self {
        let mut start = self.start;
        let inverted_map_matrix = self
            .matrix
            .grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| match cell {
                        Square::Elevation(elevation) => {
                            Square::Elevation(Square::End.elevation() - elevation + 1)
                        }
                        Square::Start => Square::End,
                        Square::End => {
                            start = (x, y);
                            Square::Start
                        }
                    })
                    .collect::<Array<_>>()
            })
            .collect::<Array<_>>();

        Self {
            matrix: Matrix::try_from(inverted_map_matrix)
                .expect("Matrix must be inverted from a valid Matrix"),
            start,
        }
    }
}

impl TryFrom<&str> for Map {
    type Error = Failure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut start = None;
        let mut found_end = false;

        let grid = value
            .split('\n')
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((x, y));
                            Square::Start
                        }
                        'E' => {
                            found_end = true;
                            Square::End
                        }
                        c => Square::Elevation(c as u8 - 96),
                    })
                    .collect::<Array<_>>()
            })
            .collect::<Array<_>>();

        if !found_end {
            return Err(e!("Map did not contain an end point"));
        }

        Ok(Self {
            matrix: Matrix::try_from(grid)?,
            start: start.ok_or_else(|| e!("Map did not contain a start point"))?,
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Direction {
    fn step_from(self, (x, y): Coord) -> Option<Coord> {
        let delta = match self {
            Direction::Up => {
                if y < 1 {
                    return None;
                }
                (x, y - 1)
            }
            Direction::Down => (x, y + 1),
            Direction::Left => {
                if x < 1 {
                    return None;
                }
                (x - 1, y)
            }
            Direction::Right => (x + 1, y),
        };

        Some(delta)
    }

    fn all() -> std::slice::Iter<'static, Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
    }

    fn others(self) -> impl Iterator<Item = Direction> {
        Direction::all().copied().filter(move |d| *d != self)
    }
}

struct MapSearcher<'a> {
    queue: VecDeque<Coord>,
    searched: Matrix<Option<Direction>>,
    map: &'a Map,
}

impl<'a> MapSearcher<'a> {
    fn init(map: &'a Map) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(map.start);

        let searched = Matrix::from_elem(None, map.matrix.width, map.matrix.height);

        Self {
            queue,
            searched,
            map,
        }
    }

    fn survey(&mut self, position: Coord, elevation: u8, direction: Direction) -> Result<()> {
        let Some(target_coord) = direction.step_from(position) else {
            return Ok(());
        };

        let Some(target) = self.map.matrix.get(target_coord).ok() else {
            return Ok(());
        };

        if self.searched.get(target_coord)?.is_some() {
            return Ok(());
        }

        if target.elevation() > elevation + 1 {
            return Ok(());
        }

        self.queue.push_back(target_coord);
        *self.searched.get_mut(target_coord)? = Some(-direction);

        Ok(())
    }

    fn search_for_end(&mut self) -> Result<Coord> {
        while let Some(coord) = self.queue.pop_front() {
            match self.map.matrix.get(coord)? {
                Square::Start => {
                    for direction in Direction::all() {
                        self.survey(coord, Square::Start.elevation(), *direction)?;
                    }
                }
                Square::Elevation(elevation) if *elevation == 26 => return Ok(coord),
                Square::Elevation(elevation) => {
                    let directions = self
                        .searched
                        .get(coord)?
                        .ok_or_else(|| e!("Expected queued Square to have been surveyed"))?
                        .others();
                    for direction in directions {
                        self.survey(coord, *elevation, direction)?;
                    }
                }
                Square::End => return Ok(coord),
            }
        }

        Err(e!("End not found"))
    }

    fn get_shortest_path(&self, mut from: Coord) -> Result<Vec<Direction>> {
        let mut path = vec![];
        loop {
            if from == self.map.start {
                return Ok(path);
            }
            let direction = self
                .searched
                .get(from)?
                .ok_or_else(|| e!("Ran out of directions!"))?;
            path.push(direction);
            from = direction
                .step_from(from)
                .ok_or_else(|| e!("Tried to lead shortest path outside the map"))?;
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "↑"),
            Self::Down => write!(f, "↓"),
            Self::Left => write!(f, "←"),
            Self::Right => write!(f, "→"),
        }
    }
}

impl Debug for MapSearcher<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.searched.grid.iter() {
            for cell in row.iter() {
                write!(
                    f,
                    "{}",
                    if let Some(direction) = cell {
                        format!("{:?}", direction)
                    } else {
                        ".".to_owned()
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn coords_from_directions(mut from: Coord, directions: &Vec<Direction>) -> Vec<Coord> {
    let mut coords = vec![];
    for direction in directions {
        coords.push(from);
        if let Some(new_from) = direction.step_from(from) {
            from = new_from;
        } else {
            return coords;
        }
    }
    coords
}

fn main() -> Result<()> {
    let input = get_input()?;

    let map = Map::try_from(input.as_str())?;
    let map = map.invert();

    let mut searcher = MapSearcher::init(&map);

    let end = searcher.search_for_end()?;

    let shortest_path = searcher.get_shortest_path(end)?;

    let mut path_visual = Matrix::from_elem(false, map.matrix.width, map.matrix.height);

    for coord in coords_from_directions(end, &shortest_path) {
        *path_visual.get_mut(coord)? = true;
    }

    println!("Map:\n{}", map);
    println!("Search map:\n{:?}", searcher);
    println!("Shortest path visual:\n{:?}", path_visual);
    println!("End: {:?}", end);
    println!(
        "Shortest path: {:?}",
        shortest_path.iter().rev().map(|d| -*d).collect::<Vec<_>>()
    );
    println!("Shortest path length: {:?}", shortest_path.len());

    Ok(())
}
