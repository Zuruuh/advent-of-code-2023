pub fn six1(input: &str) -> usize {
    let mut races = input
        .lines()
        .map(|line| {
            line.chars()
                .skip_while(|char| char.is_alphanumeric())
                .skip(1)
                .skip_while(|char| char.is_whitespace())
                .collect::<String>()
                .split(' ')
                .filter(|chunk| !chunk.trim().is_empty())
                .map(|number| number.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter();

    let races = std::iter::zip(races.next().unwrap(), races.next().unwrap()).collect::<Vec<_>>();

    races
        .into_iter()
        .map(|(time, distance)| {
            (1..time)
                .into_iter()
                .filter(|presstime| (presstime * (time - presstime)) > distance)
                .count()
        })
        .product()
}

pub fn six2(input: &str) -> usize {
    let mut races = input
        .lines()
        .map(|line| {
            line.chars()
                .skip_while(|char| char.is_alphanumeric())
                .skip(1)
                .skip_while(|char| char.is_whitespace())
                .collect::<String>()
                .split(' ')
                .filter(|chunk| !chunk.trim().is_empty())
                .map(|chunk| chunk.trim())
                .collect::<String>()
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .into_iter();

    let races = vec![(races.next().unwrap(), races.next().unwrap())];

    races
        .into_iter()
        .map(|(time, distance)| {
            (1..time)
                .into_iter()
                .filter(|presstime| (presstime * (time - presstime)) > distance)
                .count()
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::six2;

    #[test]
    pub fn test() {
        let input = include_str!("./six.txt");
        let output = six2(input);

        assert_eq!(288, output);
    }
}
