use ortalib::{Card, PokerHand, Rank};
use std::vec;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct HandValue {
    pub hand: PokerHand,
    pub cards_impl: Vec<Card>,
}

impl HandValue {
    pub fn evaluation(cards: &[Card]) -> Self {
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
        
        for checkcard in check {
            if let Some(eval) = checkcard(cards) {
                return eval;
            }
        }

        Self::check_high_card(cards).unwrap()
    }


    fn is_flush(cards: &[Card]) -> bool {
        cards.len() == 5 && {
            let base_suit = cards[0].suit;
            cards.iter().all(|c| c.suit == base_suit)
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

        let nums: Vec<u8> = cards.iter().map(|c| Self::rank_to_numeric(&c.rank)).collect();

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
    

    fn group_by_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
        let mut groups: HashMap<Rank, Vec<Card>> = HashMap::new();
        for &card in cards {
            groups.entry(card.rank)
                .or_default()
                .push(card);
        }
        groups
    }

    fn check_flush_five(cards: &[Card]) -> Option<Self> {
        let base_rank = cards[0].rank;
        if Self::is_flush(cards) && cards.iter().all(|c| c.rank == base_rank) {
            Some(Self {
                hand: PokerHand::FlushFive,
                cards_impl: cards.to_vec(),
            })
        }  else {
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
                })
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
            })

        } else {
            None
        }
    }

    fn check_straight_flush(cards: &[Card]) ->Option<Self> {
        if Self::is_flush(cards) && Self::is_straight(cards) {
            Some(Self {
                hand: PokerHand::StraightFlush,
                cards_impl: cards.to_vec(),
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

        if counts == vec![1, 4] {
            Some(Self {
                hand: PokerHand::FourOfAKind,
                cards_impl: cards.to_vec(),
            })
        } else {
            None
        }
    }
    
    fn check_full_house(cards: &[Card]) ->Option<Self> {
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
            })
        } else {
            None
        }
    }

    fn check_flush(cards: &[Card]) -> Option<Self> {
        if Self::is_flush(cards) {
            Some(Self {
                hand: PokerHand::Flush,
                cards_impl: cards.to_vec()
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
            Some(Self {
                hand: PokerHand::ThreeOfAKind,
                cards_impl: cards.to_vec(),
            })
        } else {
            None
        }
    }

    fn check_two_pair(cards: &[Card]) -> Option<Self> {
        if cards.len() < 4 {
            return None
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.iter().filter(|&&c| c == 2).count() == 2 {
            Some(Self {
                hand: PokerHand::TwoPair,
                cards_impl: cards.to_vec(),
            })
        } else {
            None
        }
    }

    fn check_pair(cards: &[Card]) ->Option<Self> {
        if cards.len() < 2 {
            return None
        }
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        if counts.iter().filter(|&&c| c == 2).count() == 1 {
            Some(Self {
                hand: PokerHand::Pair,
                cards_impl: cards.to_vec(),
            })
        } else {
            None
        }
    }
    fn check_high_card(cards: &[Card]) -> Option<Self> {
        let card_len: usize = cards.len();
        let groups = Self::group_by_rank(cards);
        let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
        counts.sort();
        if counts.len() == card_len {
            Some(Self {
                hand: PokerHand::HighCard,
                cards_impl: cards.to_vec(),
            })
        } else {
            None
        }
    }
    
}




