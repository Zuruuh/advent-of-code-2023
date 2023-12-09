pub fn five(input: &str) -> usize {
    let _seeds = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .skip(7) // Skip "seeds: "
        .collect::<String>()
        .split(' ')
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input
        .split("\n\n")
        .skip(1)
        .map(|map| {
            let map_to = {
                let map = map
                    .chars()
                    .take_while(|char| !char.is_whitespace())
                    .collect::<String>()
                    .split('-')
                    .filter(|word| word != &"to")
                    .map(|word| word.to_string())
                    .collect::<Vec<_>>();

                (map.first().cloned().unwrap(), map.last().cloned().unwrap())
            };

            let ranges = map
                .chars()
                .skip_while(|char| !char.is_whitespace())
                .skip(1)
                .collect::<String>()
                .split('\n')
                .skip(1)
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let mut numbers = line
                        .split(' ')
                        .map(|number| number.parse::<usize>().unwrap());

                    (
                        // Destination range
                        numbers.next().unwrap(),
                        // whatever this is
                        numbers.next().unwrap(),
                        numbers.next().unwrap(),
                    )
                })
                .collect::<Vec<_>>();

            (map_to.0, map_to.1, ranges)
        })
        .map(|_| 1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::five;

    #[test]
    pub fn test() {
        let input = include_str!("./five.txt");
        let output = five(input);

        assert_eq!(35, output);
    }
}
