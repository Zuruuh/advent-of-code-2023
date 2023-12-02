use std::collections::HashMap;

pub fn two(input: &str) -> usize {
    input
        .lines()
        .map(|game| {
            let game = game
                .chars()
                .skip(6) // Skip "Game: "
                .skip_while(|char| char.is_numeric()) // Skip line number
                .skip(2) // Skip ": "
                .collect::<String>();

            let mut cubes = HashMap::<String, usize>::new();
            for lot in game.split(";") {
                for pick in lot.split(", ") {
                    let count: usize = pick
                        .chars()
                        .skip_while(|char| char.is_whitespace())
                        .take_while(|char| char.is_numeric())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    let color = pick
                        .chars()
                        .skip_while(|char| char.is_numeric() || char.is_whitespace())
                        .collect::<String>();

                    let old_count = cubes.get(&color).cloned().unwrap_or_default();
                    cubes.insert(color, old_count.max(count));
                }
            }

            cubes.into_iter().map(|(_, count)| count).product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::two;

    #[test]
    pub fn test() {
        let input = include_str!("./two.txt");
        let result = two(input);

        assert_eq!(2286, result);
    }
}
