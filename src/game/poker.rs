use std::cmp::Reverse;

use arrayvec::ArrayVec;

use crate::game::Card;

/// Represents the category of a 5-card poker hand.
/// Ordered from weakest to strongest so PartialOrd/Ord work correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PokerHandCategory {
    /// No other hand category applies.
    HighCard,

    /// One pair of cards.
    OnePair,

    /// Two different pairs.
    TwoPair,

    /// Three cards of the same rank.
    ThreeOfAKind,

    /// Five consecutive cards of mixed suits.
    Straight,

    /// Five cards of the same suit, not in sequence.
    Flush,

    /// Three cards of one rank and two cards of another rank.
    FullHouse,

    /// Four cards of the same rank.
    FourOfAKind,

    /// Five consecutive cards of the same suit (excluding Royal Flush).
    StraightFlush,

    /// A, K, Q, J, 10 all of the same suit. The highest possible hand.
    RoyalFlush,
}

impl PokerHandCategory {
    pub fn num_scoring_cards(self) -> usize {
        use PokerHandCategory::*;
        match self {
            HighCard => 1,
            OnePair => 2,
            ThreeOfAKind => 3,
            TwoPair | FourOfAKind => 4,
            _ => 5,
        }
    }

    pub fn display_string(self) -> &'static str {
        use PokerHandCategory::*;
        match self {
            HighCard => "High card",
            OnePair => "Pair",
            TwoPair => "Two pairs",
            ThreeOfAKind => "3 of a kind",
            Straight => "Straight",
            Flush => "Flush",
            FullHouse => "Full house",
            FourOfAKind => "4 of a kind",
            StraightFlush => "Straight flush",
            RoyalFlush => "Royal flush",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PokerHandScore {
    pub category: PokerHandCategory,
    pub ranks: [u8; 5], 
}

pub fn score_hand(cards: [Card; 5]) -> PokerHandScore {
    // Convert internal rank (1-13) to output rank (2-14, Ace=14)
    let out_rank = |r: u8| if r == 1 { 14 } else { r };

    let mut ranks = cards.map(|c| out_rank(c.rank));
    ranks.sort_by_key(|&r| Reverse(r));

    // Check for flush (all same suit)
    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);

    // Check for straight
    let is_consecutive = ranks.windows(2).all(|w| w[0] == w[1] + 1);
    let is_wheel = ranks == [14, 5, 4, 3, 2];
    let is_broadway = is_consecutive && ranks[0] == 14;
    let is_straight = is_consecutive || is_wheel;

    let straight_ranks = if is_wheel {
        [5, 4, 3, 2, 1]  // Ace represented as 1 (low)
    } else {ranks};

    // Straight Flush / Royal Flush
    if is_flush && is_straight {
        return PokerHandScore {
            category: if is_broadway { 
                PokerHandCategory::RoyalFlush 
            } else { 
                PokerHandCategory::StraightFlush 
            },
            ranks: straight_ranks,
        };
    }

    // Count frequencies
    let mut freq = [0u8; 15]; // index 0-1 unused, 2-14 for ranks
    for card in &cards {
        freq[card.rank as usize] += 1;
    }

    let mut quads = 0u8;
    let mut trips = 0u8;
    let mut pairs = ArrayVec::<u8, 2>::new();
    let mut kickers = ArrayVec::<u8, 5>::new();

    for r in (2..=14).rev() {
        match freq[r] {
            4 => quads = r as u8,
            3 => trips = r as u8,
            2 => pairs.push(r as u8),
            1 => kickers.push(r as u8),
            _ => {}
        }
    }

    // Four of a Kind
    if quads != 0 {
        return PokerHandScore {
            category: PokerHandCategory::FourOfAKind,
            ranks: [quads, quads, quads, quads, kickers[0]],
        };
    }

    // Full House
    if trips != 0 && !pairs.is_empty() {
        return PokerHandScore {
            category: PokerHandCategory::FullHouse,
            ranks: [trips, trips, trips, pairs[0], pairs[0]],
        };
    }

    // Flush
    if is_flush {
        return PokerHandScore {
            category: PokerHandCategory::Flush,
            ranks,
        };
    }

    // Straight
    if is_straight {
        return PokerHandScore {
            category: PokerHandCategory::Straight,
            ranks: straight_ranks,
        };
    }

    // Three of a Kind
    if trips != 0 {
        return PokerHandScore {
            category: PokerHandCategory::ThreeOfAKind,
            ranks: [trips, trips, trips, kickers[0], kickers[1]],
        };
    }

    // Two Pair
    if pairs.len() == 2 {
        return PokerHandScore {
            category: PokerHandCategory::TwoPair,
            ranks: [pairs[0], pairs[0], pairs[1], pairs[1], kickers[0]],
        };
    }

    // One Pair
    if pairs.len() == 1 {
        return PokerHandScore {
            category: PokerHandCategory::OnePair,
            ranks: [pairs[0], pairs[0], kickers[0], kickers[1], kickers[2]],
        };
    }

    // High Card
    PokerHandScore {
        category: PokerHandCategory::HighCard,
        ranks,
    }
}