use std::collections::BTreeMap;

pub fn four(input: &str) -> usize {
    let mut cards = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let numbers = line
                .chars()
                .skip(4) // Skip "Card"
                .skip_while(|char| char.is_numeric() || char.is_whitespace()) // Skip " X"
                .skip(2) // Skip ": "
                .collect::<String>()
                .split(" | ")
                .map(|numbers| {
                    numbers
                        .split(' ')
                        .filter(|number| !number.is_empty())
                        .map(|number| number.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (
                i + 1,
                (
                    numbers.first().cloned().unwrap(),
                    numbers.last().cloned().unwrap(),
                    1,
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut cards_count = 0;
    let mut index = 1;

    while !cards.is_empty() {
        let (winning_numbers, played_numbers, count) = cards.remove(&index).unwrap();
        cards_count += count;

        let won_cards = played_numbers
            .iter()
            .filter(|played_number| winning_numbers.contains(played_number))
            .count();

        println!("Card {index} won {won_cards} cards");
        for card_to_add_to in 1..=won_cards {
            let new_card = cards.get(&(index + &card_to_add_to)).cloned();
            if let Some(mut new_card) = new_card {
                println!("Adding {count} card to {}", index + card_to_add_to);
                new_card.2 += count;
                cards.insert(index + card_to_add_to, new_card);
            }
        }

        index += 1;
    }

    cards_count
}

#[cfg(test)]
mod test {
    use super::four;

    #[test]
    pub fn test() {
        let input = include_str!("./four.txt");
        let output = four(input);

        assert_eq!(9496801, output);
    }
}
