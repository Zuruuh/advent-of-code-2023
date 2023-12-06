use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn surrounding(&self) -> [Position; 8] {
        let x = self.x;
        let y = self.y;

        [
            Self { x, y: y + 1 },
            Self {
                x,
                y: (y as i32 - 1).max(0) as usize,
            },
            Self { x: x + 1, y: y + 1 },
            Self {
                x: x + 1,
                y: (y as i32 - 1).max(0) as usize,
            },
            Self {
                x: (x as i32 - 1).max(0) as usize,
                y: (y as i32 - 1).max(0) as usize,
            },
            Self {
                x: (x as i32 - 1).max(0) as usize,
                y: y + 1,
            },
            Self {
                x: (x as i32 - 1).max(0) as usize,
                y,
            },
            Self { x: x + 1, y },
        ]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cell {
    Symbol,
    Number(u32),
}

impl Into<u32> for Cell {
    fn into(self) -> u32 {
        match self {
            Cell::Symbol => panic!(),
            Cell::Number(num) => num,
        }
    }
}

pub fn three(input: &str) -> usize {
    let mut grid = BTreeMap::<Position, Option<Cell>>::new();

    input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars().enumerate().for_each(|(y, char)| {
                grid.insert(
                    Position { x, y },
                    match char {
                        char if char.is_ascii_digit() => {
                            Some(Cell::Number(char.to_digit(10).unwrap()))
                        }
                        '.' => None,
                        _ => Some(Cell::Symbol),
                    },
                );
            })
        })
        .for_each(drop);

    let mut numbers = Vec::<(usize, Position)>::new();

    let mut grid_iter = grid.iter().peekable();
    while grid_iter.len() != 0 {
        let (pos, cell) = grid_iter.next().unwrap();

        if !cell
            .map(|cell| matches!(cell, Cell::Number(_)))
            .unwrap_or_default()
        {
            continue;
        }

        let cell = cell.unwrap();
        let mut current_numbers: Vec<u32> = vec![cell.into()];

        loop {
            if let Some((_, next_cell)) = grid_iter.peek() {
                if next_cell
                    .map(|cell| matches!(cell, Cell::Number(_)))
                    .unwrap_or_default()
                {
                    current_numbers.push(next_cell.unwrap().into());
                    grid_iter.next();
                } else {
                    break;
                }
            }
        }

        numbers.push((
            current_numbers
                .into_iter()
                .map(|u32| u32.to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            *pos,
        ));
    }

    numbers
        .iter()
        .filter(|(number, position)| {
            number.to_string().chars().enumerate().any(|(i, _)| {
                Position {
                    x: position.x,
                    y: position.y + i,
                }
                .surrounding()
                .iter()
                .any(|pos| matches!(grid.get(pos), Some(Some(Cell::Symbol))))
            })
        })
        .map(|(number, _)| number)
        .sum()
}

#[cfg(test)]
mod test {
    use super::three;

    #[test]
    pub fn test() {
        let input = include_str!("./three.txt");
        let result = three(input);

        assert_eq!(4361, result);
    }
}
