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
    counts: Vec<usize>,
}



impl HandJoker {
    pub fn analyze(handvalue: &HandValue) -> Self {
        let groups = HandValue::group_by_rank(&handvalue.cards_impl);
        let mut counts: Vec<_> = groups.values().map(|v| v.len()).collect();
        counts.sort();

        let origin_data = Self {
            hand: handvalue.hand.clone(),
            cards_impl: handvalue.cards_impl.clone(),
            cards_hold_in_hand: handvalue.cards_hold_in_hand.clone(),
            work_joker_cards_in_hand: Vec::new(),
            total_joker_number: 0,
            counts,
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
            Joker::Joker | Joker::AbstractJoker => {
                true
            }
            Joker::JollyJoker | Joker::SlyJoker => {
                if data.counts.last() == Some(&5)|| data.counts.last() == Some(&4) || data.counts.last() == Some(&3) || data.counts.last() == Some(&2) {
                    true
                } else {
                    false
                }
            }
            Joker::ZanyJoker | Joker::WilyJoker => {
                if data.counts.last() == Some(&5)|| data.counts.last() == Some(&4) || data.counts.last() == Some(&3) {
                    true
                } else {
                    false
                }
            }
            Joker::MadJoker | Joker::CleverJoker => {
                if data.counts == vec![2, 2] || data.counts == vec![1, 2, 2] {
                    true
                } else {
                    false
                }
            }
            Joker::CrazyJoker | Joker::DeviousJoker => {
                if data.hand == PokerHand::Straight || data.hand == PokerHand::StraightFlush {
                    true
                } else {
                    false
                }
            }
            Joker::DrollJoker | Joker::CraftyJoker => {
                let groups = HandValue::is_flush(&data.cards_impl);
                if groups {
                    true
                } else {
                    false
                }
            }
            _ => {
                false
            }
        }
    }
}