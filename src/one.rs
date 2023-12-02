const NUMBERS: [(&'static str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut numbers = Vec::<usize>::new();
            let mut line = String::from(line);

            while !line.is_empty() {
                println!("-- Checking line \"{line}\"");
                for (num_str, num) in NUMBERS.iter() {
                    if line.starts_with(num_str) {
                        println!("Found {num_str} ({num}) in line!");
                        line = line.chars().skip(1).collect();
                        numbers.push(num.clone());
                        break;
                    }
                }

                let mut chars = line.chars();
                if let Some(char) = chars.next() {
                    if char.is_numeric() {
                        println!("Found numeric char {}!", char);
                        numbers.push(char.to_string().parse::<usize>().unwrap());
                    }
                }

                line = chars.collect();
            }

            format!(
                "{}{}",
                numbers.first().cloned().unwrap(),
                numbers.last().cloned().unwrap()
            )
            .parse::<usize>()
            .map(|e| {
                println!("Submitting result {e}");
                e
            })
            .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::one;

    #[test]
    fn test() {
        let input = include_str!("./one.txt");
        let result = one(input);

        assert_eq!(54578, result);
    }
}
