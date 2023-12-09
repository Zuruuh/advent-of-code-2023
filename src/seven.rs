use std::{cmp::Ordering, collections::BTreeMap, fmt};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl From<&char> for Card {
    fn from(value: &char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Number(10),
            number if number.is_digit(10) && number != &'0' && number != &'1' => {
                Card::Number(number.to_digit(10).unwrap() as u8)
            }
            _ => unreachable!("{value}"),
        }
    }
}

impl Into<u8> for &Card {
    fn into(self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(num) => *num,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ace => 'A',
                Self::King => 'K',
                Self::Queen => 'Q',
                Self::Jack => 'J',
                Self::Number(number) => (number == &10)
                    .then(|| 'T')
                    .unwrap_or_else(|| { number.to_string().chars().next().unwrap() }),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Deck {
    pub cards: Vec<Card>,
    pub score: Score,
}

impl FromIterator<Card> for Deck {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let cards = iter.into_iter().collect::<Vec<_>>();

        Self::new(cards)
    }
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        if cards.len() != 5 {
            panic!()
        }

        Self {
            score: Score::from(&cards),
            cards,
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|card| card.to_string())
                .collect::<String>()
        )
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Score {
    // where all five cards have the same label: AAAAA
    FiveOfAKind,
    // where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    // where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    // where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    // where all cards' labels are distinct: 23456
    HighCard,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FiveOfAKind => "five of a kind",
                Self::FourOfAKind => "four of a kind",
                Self::FullHouse => "full house",
                Self::ThreeOfAKind => "three of a kind",
                Self::TwoPair => "two pair",
                Self::OnePair => "one pair",
                Self::HighCard => "high card",
            }
        )
    }
}

impl From<&Vec<Card>> for Score {
    fn from(value: &Vec<Card>) -> Self {
        let mut map = BTreeMap::<String, usize>::new();

        value.into_iter().for_each(|card| {
            let old = map.remove(&card.to_string()).unwrap_or_default();
            map.insert(card.to_string(), old + 1);
        });

        let mut map = map.into_iter().map(|u| u.1).collect::<Vec<_>>();
        map.sort();
        let mut map = map.into_iter().rev();
        let map = (
            map.next().unwrap_or_default(),
            map.next().unwrap_or_default(),
        );

        match map {
            (5, 0) => Score::FiveOfAKind,
            (4, 1) => Score::FourOfAKind,
            (3, 2) => Score::FullHouse,
            (3, 1) => Score::ThreeOfAKind,
            (2, 2) => Score::TwoPair,
            (2, 1) => Score::OnePair,
            (1, 1) => Score::HighCard,
            _ => unreachable!("{:?}", map),
        }
    }
}

pub fn seven(input: &str) -> usize {
    let mut decks = input
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .map(|(deck, bid)| {
            let deck = deck.chars().map(|char| Card::from(&char)).collect::<Deck>();
            (deck, bid)
        })
        .collect::<Vec<_>>();

    decks.sort_by(|a, b| match a.0.score.cmp(&b.0.score) {
        Ordering::Equal => {
            let cards = a.0.cards.iter().zip(b.0.cards.iter()).collect::<Vec<_>>();
            for (a, b) in cards {
                match <&Card as Into<u8>>::into(a).cmp(&b.into()) {
                    Ordering::Equal => {}
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Greater => return Ordering::Less,
                }
            }

            panic!("wtf")
        }
        cmp => cmp,
    });

    decks.reverse();

    decks
        .into_iter()
        .enumerate()
        .map(|(i, (deck, bid))| {
            println!(
                "Deck with cards {deck} and score {} has won place {} for a bid of {bid} ({})",
                deck.score,
                i + 1,
                bid * (i + 1)
            );

            (i, (deck, bid))
        })
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use super::seven;

    #[test]
    pub fn test() {
        let input = include_str!("./seven.txt");
        let output = seven(input);

        assert_eq!(6440, output);
    }
}
