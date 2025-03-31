use super::pokerhand::HandValue;
use ortalib::{Chips, Mult, PokerHand};

#[derive(Debug, Clone, PartialEq)]
pub struct Sorce {
    pub card_chips: Chips,
    pub base_chips: Chips,
    pub total_chips: Chips,
    pub mult: Mult,
}

impl Sorce {
    pub fn get_card(hand: HandValue) -> Self {
        let (card_chips, mult) = PokerHand::hand_value(&hand.hand);
        let base_chips = hand.cards_impl.iter().map(|v| v.rank.rank_value()).sum();
        Self {
            card_chips,
            base_chips,
            total_chips: card_chips + base_chips,
            mult,
        }
    }
}
