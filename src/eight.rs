use std::collections::BTreeMap;

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
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

    let mut positions = Vec::<String>::new();

    let paths = input
        .map(|line| {
            let mut line = line.chars().collect::<Vec<_>>();

            (
                line.iter().take(3).collect::<String>(),
                (
                    {
                        line = line.into_iter().skip(3 + 4).collect();
                        line.iter().take(3).collect::<String>()
                    },
                    {
                        line = line.into_iter().skip(2 + 3).collect();
                        line.iter().take(3).collect::<String>()
                    },
                ),
            )
        })
        .map(|line| {
            if line.0.ends_with('A') {
                positions.push(line.0.clone());
            }

            line
        })
        .collect::<BTreeMap<_, _>>();

    let total_steps_count = positions
        .into_iter()
        .map(|pos| {
            let mut next_position: String = pos.clone();
            let mut count = 0;
            for instruction in instructions.clone() {
                count += 1;
                let goto = paths.get(&next_position).unwrap();
                next_position = match instruction {
                    Instruction::Left => goto.0.clone(),
                    Instruction::Right => goto.1.clone(),
                };

                if next_position.ends_with('Z') {
                    break;
                }
            }

            count
        })
        .fold(1, lcm);

    total_steps_count
}

/// I stole this function online
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp_a = a;
        a = b;
        b = tmp_a % b;
    }

    return a;
}

#[cfg(test)]
mod test {
    use super::eight;

    #[test]
    pub fn test() {
        let input = include_str!("./eight.txt");
        let output = eight(input);

        assert_eq!(17972669116327, output);
    }
}
