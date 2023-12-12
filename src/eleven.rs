use std::collections::BTreeMap;

use crate::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Galaxy,
    Space,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Galaxy => '#',
                _ => '.',
            }
        )
    }
}

pub fn eleven(input: &str) -> isize {
    let original_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Cell::from(char))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid = Vec::<Vec<Cell>>::new();
    for _ in 0..original_grid.len() {
        grid.push(Vec::new());
    }

    // vertical
    for (i, _) in original_grid.iter().enumerate() {
        let line = original_grid
            .iter()
            .map(|vec| *vec.get(i).unwrap())
            .collect::<Vec<_>>();
        let is_only_spaces = line.iter().all(|cell| matches!(cell, Cell::Space));

        if is_only_spaces {
            grid.iter_mut().for_each(|vec| {
                vec.push(Cell::Space);
                vec.push(Cell::Space);
            })
        } else {
            for (j, cell) in line.into_iter().enumerate() {
                grid.get_mut(j).unwrap().push(cell);
            }
        }
    }

    // horizontal
    let grid = grid
        .into_iter()
        .flat_map(|line| {
            line.iter()
                .all(|cell| matches!(cell, Cell::Space))
                .then(|| vec![line.clone(), line.clone()])
                .unwrap_or_else(|| vec![line])
        })
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, cell)| (Position { x, y }, *cell))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();

    let galaxies = grid
        .iter()
        .filter(|(_, cell)| matches!(cell, Cell::Galaxy))
        .collect::<BTreeMap<_, _>>();

    let mut count: isize = 0;

    {
        let mut expanded_grid = String::new();
        for line in grid
            .iter()
            .collect::<Vec<_>>()
            .group_by(|a, b| a.0.x == b.0.x)
        {
            for cell in line {
                expanded_grid.push_str(&cell.1.to_string());
            }

            expanded_grid.push('\n');
        }
        print!("{expanded_grid}");
    }

    for (i, (pos, _)) in galaxies.iter().enumerate() {
        for (next_pos, _) in galaxies.iter().skip(i + 1) {
            let distance = (next_pos.x.max(pos.x) as isize - pos.x.min(next_pos.x) as isize)
                + (next_pos.y.max(pos.y) as isize - pos.y.min(next_pos.y) as isize);

            count += distance;

            println!("Calculated a distance of {distance} between {pos} and {next_pos}");
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::eleven;

    #[test]
    pub fn test() {
        let input = include_str!("./eleven.txt");
        let output = eleven(input);

        assert_eq!(374, output);
    }
}
