pub fn nine(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut numbers = vec![line
                .split(' ')
                .map(|number| number.parse::<isize>().unwrap())
                .collect::<Vec<_>>()];

            loop {
                let current_numbers = numbers.last().unwrap();
                if current_numbers.iter().all(|number| number == &0) {
                    break;
                }

                numbers.push(
                    current_numbers
                        .iter()
                        .map_windows(|[a, b]| *b - *a)
                        .collect::<Vec<_>>(),
                );
            }

            *numbers
                .into_iter()
                .rev()
                .reduce(|acc, numbers| match acc.last() {
                    None => vec![0],
                    Some(last) => {
                        let mut acc = acc.clone();
                        acc.push(numbers.last().unwrap() + last);
                        acc
                    }
                })
                .unwrap()
                .last()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::nine;

    #[test]
    pub fn test() {
        let input = include_str!("./nine.txt");
        let output = nine(input);

        assert_eq!(1702218515, output);
    }
}
