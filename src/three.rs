use std::collections::{BTreeMap, HashSet};

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
    Number(u32),
    Star,
}

impl Into<u32> for Cell {
    fn into(self) -> u32 {
        match self {
            Cell::Star => panic!(),
            Cell::Number(num) => num,
        }
    }
}

pub fn three(input: &str) -> usize {
    let mut grid = BTreeMap::<Position, Option<Cell>>::new();
    let mut stars = Vec::<Position>::new();

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
                        '*' => {
                            stars.push(Position { x, y });
                            Some(Cell::Star)
                        }
                        _ => None,
                    },
                );
            })
        })
        .for_each(drop);

    let mut numbers = Vec::<(usize, Vec<Position>)>::new();

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

        let number = current_numbers
            .into_iter()
            .map(|u32| u32.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        numbers.push((
            number,
            number
                .to_string()
                .chars()
                .enumerate()
                .map(|(i, _)| Position {
                    x: pos.x,
                    y: pos.y + i,
                })
                .collect::<Vec<_>>(),
        ));
    }

    stars
        .iter()
        .map(|position| {
            let adjacent_numbers = position
                .surrounding()
                .iter()
                .map(|surrounding_position| (grid.get(&surrounding_position), surrounding_position))
                .filter_map(|(cell, surrounding_position)| {
                    if cell.is_none() {
                        return None;
                    }

                    let cell = cell.unwrap();
                    if cell.is_none() {
                        return None;
                    }

                    let cell = cell.unwrap();

                    match cell {
                        Cell::Number(_) => numbers
                            .iter()
                            .find(|(_, positions)| positions.contains(surrounding_position))
                            .map(|(number, _)| number),
                        Cell::Star => None,
                    }
                })
                .map(|number| number.clone())
                .collect::<HashSet<_>>();

            if adjacent_numbers.len() == 2 {
                return adjacent_numbers.iter().product();
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::three;

    #[test]
    pub fn test() {
        let input = include_str!("./three.txt");
        let result = three(input);

        assert_eq!(84584891, result);
    }
}
