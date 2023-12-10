use std::{collections::BTreeMap, fmt};

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
        }
    }
}

impl FromIterator<(Position, Option<Pipe>)> for PipeIterator {
    fn from_iter<T: IntoIterator<Item = (Position, Option<Pipe>)>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<BTreeMap<_, _>>())
    }
}

impl Iterator for PipeIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.starting_position == self.current_position {
            self.current_position = self
                .starting_position
                .surrounding_without_diagonals()
                .iter()
                .find_map(|pos| match self.grid.get(&pos) {
                    None => None,
                    Some(_) => Some(*pos),
                })
                .expect("Could not find a valid pipe next to starting position");

            return Some(self.current_position);
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

                Some(next_position)
            }
            None => None,
        }
    }
}

pub fn ten(input: &str) -> usize {
    (input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, char)| (Position { x, y }, Pipe::from_char(char)))
                .collect::<Vec<_>>()
        })
        .collect::<PipeIterator>()
        .into_iter()
        .count()
        / 2)
        + 1
}

#[cfg(test)]
mod test {
    use super::ten;

    #[test]
    pub fn test() {
        let input = include_str!("./ten.txt");
        let output = ten(input);

        assert_eq!(4, output);
    }
}
