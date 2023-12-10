use std::collections::BTreeMap;

use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Start => 'S',
                Pipe::Vertical => '|',
                Pipe::Horizontal => '-',
                Pipe::BottomLeft => 'L',
                Pipe::BottomRight => 'J',
                Pipe::TopLeft => 'F',
                Pipe::TopRight => '7',
            }
        )
    }
}

impl Pipe {
    pub fn from_char(char: char) -> Option<Self> {
        Some(match char {
            'S' => Self::Start,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            'L' => Self::BottomLeft,
            'J' => Self::BottomRight,
            'F' => Self::TopLeft,
            '7' => Self::TopRight,
            _ => return None,
        })
    }

    pub fn surrounding(&self, position: &Position) -> Vec<Position> {
        match self {
            Pipe::Vertical => vec![position.next_x(), position.prev_x()],
            Pipe::Horizontal => vec![position.next_y(), position.prev_y()],
            Pipe::BottomLeft => vec![position.prev_x(), position.next_y()],
            Pipe::BottomRight => vec![position.prev_x(), position.prev_y()],
            Pipe::TopLeft => vec![position.next_x(), position.next_y()],
            Pipe::TopRight => vec![position.next_x(), position.prev_y()],
            _ => panic!(""),
        }
    }
}

#[derive(Clone)]
struct PipeIterator {
    grid: BTreeMap<Position, Option<Pipe>>,
    starting_position: Position,
    current_position: Position,
    previous_position: Position,
    started: bool,
}

impl PipeIterator {
    pub fn new(grid: BTreeMap<Position, Option<Pipe>>) -> Self {
        let starting_position = **grid
            .iter()
            .filter_map(|(pos, pipe)| matches!(pipe, Some(Pipe::Start)).then(|| pos))
            .collect::<Vec<_>>()
            .first()
            .expect("Could not find starting square");

        Self {
            grid,
            starting_position,
            current_position: starting_position,
            previous_position: starting_position,
            started: false,
        }
    }
}

impl FromIterator<(Position, Option<Pipe>)> for PipeIterator {
    fn from_iter<T: IntoIterator<Item = (Position, Option<Pipe>)>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<BTreeMap<_, _>>())
    }
}

impl Iterator for PipeIterator {
    type Item = (Position, Pipe);

    fn next(&mut self) -> Option<Self::Item> {
        if self.starting_position == self.current_position {
            if self.started == false {
                self.started = true;

                return Some((
                    self.starting_position,
                    self.grid.get(&self.starting_position).unwrap().unwrap(),
                ));
            }

            self.current_position = self
                .starting_position
                .surrounding_without_diagonals()
                .iter()
                .find_map(|pos| match self.grid.get(&pos) {
                    None => None,
                    Some(pipe) => match pipe {
                        None => None,
                        Some(pipe) => pipe
                            .surrounding(pos)
                            .contains(&self.starting_position)
                            .then(|| *pos),
                    },
                })
                .expect("Could not find a valid pipe next to starting position");

            return Some((
                self.current_position,
                self.grid.get(&self.current_position).unwrap().unwrap(),
            ));
        }

        let current_pipe = self
            .grid
            .get(&self.current_position)
            .expect("Position is not contained in grid")
            .expect("Should not be currently on an empty cell");

        let next_position = current_pipe
            .surrounding(&self.current_position)
            .into_iter()
            .find(|pos| pos != &self.previous_position && pos != &self.starting_position);

        match next_position {
            Some(next_position) => {
                self.previous_position = self.current_position;
                self.current_position = next_position;

                Some((
                    next_position,
                    self.grid.get(&next_position).unwrap().unwrap(),
                ))
            }
            None => None,
        }
    }
}

pub fn ten(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, char)| (Position { x, y }, Pipe::from_char(char)))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();

    let pipes = grid
        .clone()
        .into_iter()
        .collect::<PipeIterator>()
        .collect::<BTreeMap<_, _>>();

    grid.into_iter()
        .collect::<Vec<_>>()
        .group_by(|a, b| a.0.x == b.0.x)
        .into_iter()
        .flat_map(|data| {
            let mut inside = false;

            data.into_iter()
                .filter(|(pos, cell)| match cell {
                    None => inside,
                    Some(pipe) => match pipes.contains_key(pos) {
                        false => inside,
                        true => {
                            match pipe {
                                Pipe::Horizontal | Pipe::BottomRight | Pipe::BottomLeft => {}
                                _ => {
                                    inside = !inside;
                                }
                            }

                            false
                        }
                    },
                })
                .collect::<Vec<_>>()
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::ten;

    #[test]
    pub fn test() {
        let input = include_str!("./ten.txt");
        let output = ten(input);

        assert_eq!(265, output);
    }
}
