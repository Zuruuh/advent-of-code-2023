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

enum PipeSurroundingPositionResult {
    PotentialPositions(Vec<Position>),
    NotComputable,
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

    pub fn surrounding(&self, position: &Position) -> PipeSurroundingPositionResult {
        PipeSurroundingPositionResult::PotentialPositions(match self {
            Pipe::Start => return PipeSurroundingPositionResult::NotComputable,
            Pipe::Vertical => vec![position.next_x(), position.prev_x()],
            Pipe::Horizontal => vec![position.next_y(), position.prev_y()],
            Pipe::BottomLeft => vec![position.prev_x(), position.next_y()],
            Pipe::BottomRight => vec![position.prev_x(), position.prev_y()],
            Pipe::TopLeft => vec![position.next_x(), position.next_y()],
            Pipe::TopRight => vec![position.next_x(), position.prev_y()],
        })
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

    let starting_position = **grid
        .iter()
        .filter_map(|(pos, pipe)| matches!(pipe, Some(Pipe::Start)).then(|| pos))
        .collect::<Vec<_>>()
        .first()
        .expect("Could not find starting square");

    let mut distance_grid = BTreeMap::<Position, usize>::new();
    // distance_grid.insert(*&starting_position, 0);
    explore(
        (&starting_position, &Pipe::Start),
        &grid,
        &mut distance_grid,
        0,
    );

    1
}

fn explore(
    cell: (&Position, &Pipe),
    grid: &BTreeMap<Position, Option<Pipe>>,
    distance_grid: &mut BTreeMap<Position, usize>,
    index: usize,
) {
    // for surrounding_position in cell.0.surrounding() {
    // match grid.get(&surrounding_position) {
    //     None => continue,
    //     Some(maybe_surrounding_pipe) => match maybe_surrounding_pipe {
    //         None => continue,
    //         Some(surrounding_pipe) => match surrounding_pipe {
    //             Pipe::Start => { /*TODO: Check if current cell can go to start*/ }
    //             Pipe::Vertical => {}
    //             _ => {}
    //         },
    //     },
    // }
    // }

    match cell.1.surrounding(cell.0) {
        PipeSurroundingPositionResult::NotComputable => todo!(),
        PipeSurroundingPositionResult::PotentialPositions(positions) => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::ten;

    #[test]
    pub fn test() {
        let input = include_str!("./ten.txt");
        let output = ten(input);

        assert_eq!(1, output);
    }
}
