use super::pokerhand::HandValue;
use ortalib::{Chips, Edition, Enhancement, Mult, PokerHand};

#[derive(Debug, Clone, PartialEq)]
pub struct Sorce {
    pub card_chips: Chips,
    pub total_chips: Chips,
    pub mult: Mult,
}

impl Sorce {
    pub fn get_card(hand: HandValue) -> Self {
        let (card_chips, mut mult) = PokerHand::hand_value(&hand.hand);
        let mut total_chips: f64 = 0.0;
        total_chips += card_chips;

        for card in &hand.cards_impl {
            let rank_value = card.rank.rank_value();
            if let Some(enhancement) = card.enhancement {
                match enhancement {
                    Enhancement::Bonus => {
                        total_chips += rank_value + 30.0;
                    }
                    Enhancement::Mult => {
                        mult += 4.0;
                        total_chips += rank_value;
                    }
                    Enhancement::Glass => {
                        mult *= 2.0;
                        total_chips += rank_value;
                    }
                    _  => {
                        total_chips += rank_value;
                    }
                }
            } else {
                total_chips += rank_value;
            }

            if let Some(edit) = card.edition {
                match edit {
                    Edition::Foil => {
                        total_chips += 50.0;
                    }
                    Edition::Holographic => {
                        mult += 10.0;
                    }
                    Edition::Polychrome => {
                        mult *= 1.5
                    }
                }
            }
        }
        Self {
            card_chips,
            total_chips,
            mult,
        }
    }
}
