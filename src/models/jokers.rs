use std::vec;

use ortalib::{Card, Joker, JokerCard, PokerHand};
use crate::HandValue;

#[derive(Debug)]
pub struct HandJoker{
    pub hand: PokerHand,
    pub cards_impl: Vec<Card>,
    pub cards_hold_in_hand: Vec<Card>,
    pub work_joker_cards_in_hand: Vec<JokerCard>,
    pub total_joker_number: usize,
}



impl HandJoker {
    pub fn analyze(handvalue: &HandValue) -> Self {
        let origin_data = Self {
            hand: handvalue.hand.clone(),
            cards_impl: handvalue.cards_impl.clone(),
            cards_hold_in_hand: handvalue.cards_hold_in_hand.clone(),
            work_joker_cards_in_hand: Vec::new(),
            total_joker_number: 0,
        };

        let work_joker = handvalue.joker_cards
            .iter()
            .filter(|j| Self::is_joker_work(&j.joker, &origin_data))
            .cloned()
            .collect();

        Self {
            work_joker_cards_in_hand: work_joker,
            total_joker_number: handvalue.joker_cards.len(),
            ..origin_data
        }
    }


    fn is_joker_work(joker: &Joker, data: &Self) -> bool {
        match joker {
            Joker::Joker => {
                true
            }
            Joker::JollyJoker => {
                let groups = HandValue::group_by_rank(&data.cards_impl);
                let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
                counts.sort();

                if counts.last() == Some(&5)|| counts.last() == Some(&4) || counts.last() == Some(&3) || counts.last() == Some(&2) {
                    true
                } else {
                    false
                }
            }
            Joker::ZanyJoker => {
                let groups = HandValue::group_by_rank(&data.cards_impl);
                let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
                counts.sort();

                if counts.last() == Some(&5)|| counts.last() == Some(&4) || counts.last() == Some(&3) {
                    true
                } else {
                    false
                }
            }
            Joker::MadJoker => {
                let groups = HandValue::group_by_rank(&data.cards_impl);
                let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
                counts.sort();

                if counts == vec![2, 2] || counts == vec![1, 2, 2] {
                    true
                } else {
                    false
                }
            }
            Joker::CrazyJoker => {
                if data.hand == PokerHand::Straight || data.hand == PokerHand::StraightFlush {
                    true
                } else {
                    false
                }
            }
            Joker::DrollJoker => {
                if data.hand == PokerHand::Flush || data.hand == PokerHand::FlushFive || data.hand == PokerHand::FlushHouse {
                    true
                } else {
                    false
                }
            }
            Joker::SlyJoker => {
                let groups = HandValue::group_by_rank(&data.cards_impl);
                let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
                counts.sort();

                if counts.last() == Some(&5)|| counts.last() == Some(&4) || counts.last() == Some(&3) || counts.last() == Some(&2) {
                    true
                } else {
                    false
                }
            }
            Joker::WilyJoker => {
                let groups = HandValue::group_by_rank(&data.cards_impl);
                let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
                counts.sort();

                if counts.last() == Some(&5)|| counts.last() == Some(&4) || counts.last() == Some(&3) {
                    true
                } else {
                    false
                }
            }
            Joker::CleverJoker => {
                let groups = HandValue::group_by_rank(&data.cards_impl);
                let mut counts: Vec<usize> = groups.values().map(|v| v.len()).collect();
                counts.sort();

                if counts == vec![2, 2] || counts == vec![1, 2, 2] {
                    true
                } else {
                    false
                }
            }
            Joker::DeviousJoker => {
                if data.hand == PokerHand::Straight || data.hand == PokerHand::StraightFlush {
                    true
                } else {
                    false
                }
            }
            Joker::CraftyJoker => {
                if data.hand == PokerHand::Flush || data.hand == PokerHand::FlushFive || data.hand == PokerHand::FlushHouse {
                    true
                } else {
                    false
                }
            }
            Joker::AbstractJoker => {
                true
            }
            _ => {
                false
            }
        }
    }
}