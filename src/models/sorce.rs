use super::jokers::HandJoker;
use ortalib::{Chips, Edition, Enhancement, Joker, Mult, PokerHand, Rank, Suit};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Sorce {
    pub card_chips: Chips,
    pub total_chips: Chips,
    pub mult: Mult,
}

impl Sorce {
    pub fn get_card(hand: HandJoker) -> Self {
        let (card_chips, mut mult) = PokerHand::hand_value(&hand.hand);
        let mut photograph_trigger = true;
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
                    _ => {
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
                    Edition::Polychrome => mult *= 1.5,
                }
            }
            for joker_card in &hand.work_joker_cards_in_hand {
                match joker_card.joker {
                    Joker::GreedyJoker => {
                        if card.suit == Suit::Diamonds {
                            mult += 3.0
                        }
                    }
                    Joker::LustyJoker => {
                        if card.suit == Suit::Hearts {
                            mult += 3.0
                        }
                    }
                    Joker::WrathfulJoker => {
                        if card.suit == Suit::Spades {
                            mult += 3.0
                        }
                    }
                    Joker::GluttonousJoker => {
                        if card.suit == Suit::Clubs {
                            mult += 3.0
                        }
                    }
                    Joker::Fibonacci => {
                        if card.rank.rank_value() == 2.0
                            || card.rank.rank_value() == 3.0
                            || card.rank.rank_value() == 5.0
                            || card.rank.rank_value() == 8.0
                            || card.rank.rank_value() == 11.0
                        {
                            mult += 8.0
                        }
                    }
                    Joker::ScaryFace => {
                        if card.rank == Rank::Jack
                            || card.rank == Rank::Queen
                            || card.rank == Rank::King
                        {
                            total_chips += 30.0
                        }
                    }
                    Joker::EvenSteven => {
                        if card.rank.rank_value() == 10.0
                            || card.rank.rank_value() == 8.0
                            || card.rank.rank_value() == 6.0
                            || card.rank.rank_value() == 4.0
                            || card.rank.rank_value() == 2.0
                        {
                            mult += 4.0
                        }
                    }
                    Joker::OddTodd => {
                        if card.rank.rank_value() == 11.0
                            || card.rank.rank_value() == 9.0
                            || card.rank.rank_value() == 7.0
                            || card.rank.rank_value() == 5.0
                            || card.rank.rank_value() == 3.0
                        {
                            mult += 4.0
                        }
                    }
                    Joker::Photograph => {
                        if (card.rank == Rank::Jack
                            || card.rank == Rank::Queen
                            || card.rank == Rank::King)
                            && photograph_trigger
                        {
                            mult *= 2.0;
                            photograph_trigger = false;
                        }
                    }
                    Joker::SmileyFace => {
                        if card.rank == Rank::Jack
                            || card.rank == Rank::Queen
                            || card.rank == Rank::King
                        {
                            mult += 5.0
                        }
                    }
                    _ => {}
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
                Joker::Baron => {
                    let counts = hand
                        .cards_hold_in_hand
                        .iter()
                        .filter(|c| c.rank == Rank::King)
                        .count() as f64;
                    if counts != 0.0 {
                        mult *= 1.5f64.powi(counts as i32);
                    }
                }
                _ => {}
            }
        }
        for joker_card in &hand.work_joker_cards_in_hand {
            match joker_card.joker {
                Joker::Joker => {
                    mult += 4.0;
                }
                Joker::JollyJoker => mult += 8.0,
                Joker::ZanyJoker => mult += 12.0,
                Joker::MadJoker => mult += 10.0,
                Joker::CrazyJoker => mult += 12.0,
                Joker::DrollJoker => mult += 10.0,
                Joker::SlyJoker => total_chips += 50.0,
                Joker::WilyJoker => total_chips += 100.0,
                Joker::CleverJoker => total_chips += 80.0,
                Joker::DeviousJoker => total_chips += 100.0,
                Joker::CraftyJoker => total_chips += 80.0,
                Joker::AbstractJoker => {
                    mult += 3.0 * (hand.total_joker_number as f64);
                },
                Joker::RaisedFist => {
                    let len_card_hold_in_hand = hand.cards_hold_in_hand.len();
                    let last_one = hand.cards_hold_in_hand[len_card_hold_in_hand - 1].rank;
                    mult += 2.0 * last_one.rank_value()
                }
                Joker::Blackboard => {
                    if hand
                        .cards_hold_in_hand
                        .iter()
                        .all(|c| c.suit == Suit::Clubs || c.suit == Suit::Spades)
                    {
                        mult *= 3.0
                    }
                }
                Joker::FlowerPot => {
                    if hand.cards_impl.len() >= 4 {
                        let mut fixed_suits = HashSet::new();
                        let mut flexible_count = 0;

                        for card in &hand.cards_impl {
                            if matches!(card.enhancement, Some(Enhancement::Wild)) {
                                flexible_count += 1;
                            } else {
                                fixed_suits.insert(card.suit);
                            }
                        }
                        let missing_suits = 4 - fixed_suits.len();

                        if missing_suits <= 0 || flexible_count >= missing_suits {
                            mult *= 3.0
                        }
                    }
                }
                _ => {}
            }
        }

        for hold_joker_card in &hand.joker_card {
            if let Some(editions) = hold_joker_card.edition {
                match editions {
                    Edition::Foil => {
                        total_chips += 50.0;
                    }
                    Edition::Holographic => {
                        mult += 10.0;
                    }
                    Edition::Polychrome => mult *= 1.5,
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
