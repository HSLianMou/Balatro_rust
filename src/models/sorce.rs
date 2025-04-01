use super::jokers::HandJoker;
use ortalib::{Chips, Edition, Enhancement, Joker, Mult, PokerHand};

#[derive(Debug, Clone, PartialEq)]
pub struct Sorce {
    pub card_chips: Chips,
    pub total_chips: Chips,
    pub mult: Mult,
}

impl Sorce {
    pub fn get_card(hand: HandJoker) -> Self {
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
        for hold_card in &hand.cards_hold_in_hand {
            if let Some(enhancement) = hold_card.enhancement {
                match enhancement {
                    Enhancement::Steel => {
                        mult *= 1.5;
                    }
                    _ => {}
                }
            }
        }
        for joker_card in &hand.work_joker_cards_in_hand {
            match joker_card.joker {
                Joker::Joker => {
                    mult += 4.0;
                }
                Joker::JollyJoker => {
                    mult += 8.0
                }
                Joker::ZanyJoker => {
                    mult += 12.0
                }
                Joker::MadJoker => {
                    mult += 10.0
                }
                Joker::CrazyJoker => {
                    mult += 12.0
                }
                Joker::DrollJoker => {
                    mult += 10.0
                }
                Joker::SlyJoker => {
                    total_chips += 50.0
                }
                Joker::WilyJoker => {
                    total_chips += 100.0
                }
                Joker::CleverJoker => {
                    total_chips += 80.0
                }
                Joker::DeviousJoker => {
                    total_chips += 100.0
                }
                Joker::CraftyJoker => {
                    total_chips += 80.0
                }
                Joker::AbstractJoker =>{
                    mult = mult * 3.0 * (hand.total_joker_number as f64)
                }
                _ => {}
            }
        }
        for hold_joker_card in &hand.work_joker_cards_in_hand {
            if let Some(editions) = hold_joker_card.edition {
                match editions {
                    Edition::Foil => {
                        total_chips += 50.0
                    }
                    Edition::Holographic => {
                        mult += 10.0
                    }
                    Edition::Polychrome => {
                        mult *= 1.5;
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
