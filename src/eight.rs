use std::collections::HashMap;

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("wtf"),
        }
    }
}

pub fn eight(input: &str) -> usize {
    let mut input = input.lines();
    let instructions = input
        .next()
        .unwrap()
        .chars()
        .map(|char| Instruction::from(char))
        .cycle();

    input.next().unwrap();

    let paths = input
        .map(|line| {
            let line = line.chars();

            // TODO refactor this bcz it's trash
            (
                line.clone().take(3).collect::<String>(),
                (
                    {
                        let line = line.clone().skip(4 + 3);
                        line.clone().take(3).collect::<String>()
                    },
                    {
                        let line = line.skip(4 + 3 + 2 + 3);
                        line.take(3).collect::<String>()
                    },
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    let mut positions = paths
        .iter()
        .filter_map(|(position, _)| {
            if position.ends_with('A') {
                return Some(position.clone());
            }

            None
        })
        .collect::<Vec<_>>();

    for instruction in instructions.into_iter() {
        count += 1;
        positions = positions
            .into_iter()
            .map(|position| {
                let goto = paths.get(&position);

                if goto.is_none() {
                    return position;
                }

                let goto = match instruction {
                    Instruction::Left => goto.unwrap().0.clone(),
                    Instruction::Right => goto.unwrap().1.clone(),
                };

                println!("Navigating from {position} to {goto}");

                goto
            })
            .collect();

        if positions.iter().all(|position| position.ends_with('Z')) {
            break;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::eight;

    #[test]
    pub fn test() {
        let input = include_str!("./eight.txt");
        let output = eight(input);

        assert_eq!(6, output);
    }
}
