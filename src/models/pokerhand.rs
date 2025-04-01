use ortalib::{Card, Enhancement, JokerCard, PokerHand, Rank};
use std::collections::{HashMap, HashSet};
use std::vec;

#[derive(Debug)]
pub struct HandValue {
    pub hand: PokerHand,
    pub cards_impl: Vec<Card>,
    pub cards_hold_in_hand: Vec<Card>,
    pub joker_cards: Vec<JokerCard>,
}

impl HandValue {
    pub fn evaluation(cards: &[Card], hold_cards: &[Card], joker_card: &[JokerCard]) -> Self {
        let check: Vec<fn(&[Card]) -> Option<Self>> = vec![
            Self::check_flush_five,
            Self::check_flush_house,
            Self::check_five_of_a_kind,
            Self::check_straight_flush,
            Self::check_four_of_a_kind,
            Self::check_full_house,
            Self::check_flush,
            Self::check_straight,
            Self::check_three_of_a_kind,
            Self::check_two_pair,
            Self::check_pair,
        ];

        for checkcard in &check {
            if let Some(mut eval) = checkcard(cards) {
                eval.cards_hold_in_hand = hold_cards.to_vec();
                eval.joker_cards = joker_card.to_vec();
                return eval;
            }
        }

        let mut default_eval = Self::check_high_card(cards).unwrap();
        default_eval.cards_hold_in_hand = hold_cards.to_vec();
        default_eval.joker_cards = joker_card.to_vec();
        default_eval
    }

    pub fn is_flush(cards: &[Card]) -> bool {
        let (normal_card, wild_card): (Vec<&Card>, Vec<&Card>) = cards
        .iter()
        .partition(|c| !matches!(c.enhancement, Some(Enhancement::Wild)));
        if wild_card.is_empty() {
            let base_suit = cards[0].suit;
            cards.iter().all(|c| c.suit == base_suit);
        }
        if let Some(first_normal) = normal_card.first() {
            let base_suit = first_normal.suit;
            normal_card.iter().all(|c| c.suit == base_suit)
        } else {
            false
        }
    }

    fn rank_to_numeric(rank: &Rank) -> u8 {
        match rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    fn is_straight(cards: &[Card]) -> bool {
        if cards.len() != 5 {
            return false;
        }

        let nums: Vec<u8> = cards
            .iter()
            .map(|c| Self::rank_to_numeric(&c.rank))
            .collect();

        let unique_nums: HashSet<u8> = nums.iter().cloned().collect();
        if unique_nums.len() != 5 {
            return false;
        }

        let mut sorted: Vec<u8> = unique_nums.into_iter().collect();
        sorted.sort_unstable();

        let is_normal = sorted[4] - sorted[0] == 4;

        let is_special = sorted == vec![2, 3, 4, 5, 14];

        is_normal || is_special
    }

    pub fn group_by_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
        let mut groups: HashMap<Rank, Vec<Card>> = HashMap::new();
        for &card in cards {
            groups.entry(card.rank).or_default().push(card);
        }
        groups
    }

    fn check_flush_five(cards: &[Card]) -> Option<Self> {
        let base_rank = cards[0].rank;
        if Self::is_flush(cards) && cards.iter().all(|c| c.rank == base_rank) {
            Some(Self {
                hand: PokerHand::FlushFive,
                cards_impl: cards.to_vec(),
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_flush_house(cards: &[Card]) -> Option<Self> {
        if Self::is_flush(cards) {
            let groups = Self::group_by_rank(cards);
            let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
            counts.sort();

            if counts == vec![2, 3] {
                return Some(Self {
                    hand: PokerHand::FlushHouse,
                    cards_impl: cards.to_vec(),
                    cards_hold_in_hand: Vec::new(),
                    joker_cards: Vec::new(),
                });
            } else {
                return None;
            }
        }
        None
    }

    fn check_five_of_a_kind(cards: &[Card]) -> Option<Self> {
        if cards.len() != 5 {
            return None;
        }
        let base_rank = cards[0].rank;
        if cards.iter().all(|c| c.rank == base_rank) {
            Some(Self {
                hand: PokerHand::FiveOfAKind,
                cards_impl: cards.to_vec(),
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_straight_flush(cards: &[Card]) -> Option<Self> {
        if Self::is_flush(cards) && Self::is_straight(cards) {
            Some(Self {
                hand: PokerHand::StraightFlush,
                cards_impl: cards.to_vec(),
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_four_of_a_kind(cards: &[Card]) -> Option<Self> {
        if cards.len() < 4 {
            return None;
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.last() == Some(&4) {
            let four_rank = groups
                .iter()
                .find(|(_, v)| v.len() == 4)
                .map(|(k, _)| *k)
                .expect("find_error");
            let four_cards = groups.get(&four_rank).unwrap().clone();

            Some(Self {
                hand: PokerHand::FourOfAKind,
                cards_impl: four_cards,
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_full_house(cards: &[Card]) -> Option<Self> {
        if cards.len() != 5 {
            return None;
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts == vec![2, 3] {
            Some(Self {
                hand: PokerHand::FullHouse,
                cards_impl: cards.to_vec(),
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_flush(cards: &[Card]) -> Option<Self> {
        if Self::is_flush(cards) {
            Some(Self {
                hand: PokerHand::Flush,
                cards_impl: cards.to_vec(),
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_straight(cards: &[Card]) -> Option<Self> {
        if Self::is_straight(cards) {
            Some(Self {
                hand: PokerHand::Straight,
                cards_impl: cards.to_vec(),
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_three_of_a_kind(cards: &[Card]) -> Option<Self> {
        if cards.len() < 3 {
            return None;
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.last() == Some(&3) {
            let three_rank = groups
                .iter()
                .find(|(_, v)| v.len() == 3)
                .map(|(k, _)| *k)
                .expect("find error");
            let three_cards = groups.get(&three_rank).unwrap().clone();

            Some(Self {
                hand: PokerHand::ThreeOfAKind,
                cards_impl: three_cards,
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_two_pair(cards: &[Card]) -> Option<Self> {
        if cards.len() < 4 {
            return None;
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.iter().filter(|&&c| c == 2).count() == 2 {
            let two_pairs: Vec<Card> = groups
                .values()
                .filter(|v| v.len() == 2)
                .take(2)
                .flat_map(|v| v.clone())
                .collect();

            Some(Self {
                hand: PokerHand::TwoPair,
                cards_impl: two_pairs,
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }

    fn check_pair(cards: &[Card]) -> Option<Self> {
        if cards.len() < 2 {
            return None;
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.last() == Some(&2) {
            let pair = groups
                .iter()
                .find(|(_, v)| v.len() == 2)
                .map(|(k, _)| *k)
                .expect("find error");
            let pair = groups.get(&pair).unwrap().clone();

            Some(Self {
                hand: PokerHand::Pair,
                cards_impl: pair,
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }
    fn check_high_card(cards: &[Card]) -> Option<Self> {
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.iter().all(|&c| c == 1) {
            let max_card = cards.iter().max_by_key(|c| c.rank).cloned().unwrap();

            Some(Self {
                hand: PokerHand::HighCard,
                cards_impl: vec![max_card],
                cards_hold_in_hand: Vec::new(),
                joker_cards: Vec::new(),
            })
        } else {
            None
        }
    }
}
