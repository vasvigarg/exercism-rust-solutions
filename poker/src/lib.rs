use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: u8, // 2-14 (Jack=11, Queen=12, King=13, Ace=14)
    suit: Suit,
}

// HandRank enum ordered from weakest to strongest for proper Ord implementation
#[derive(Debug, Clone, PartialEq, Eq)]
enum HandRank {
    HighCard(Vec<u8>),
    OnePair(u8, Vec<u8>),
    TwoPair(u8, u8, u8), // higher pair, lower pair, kicker
    ThreeOfAKind(u8, Vec<u8>),
    Straight(u8), // highest card (5 for A-2-3-4-5 wheel)
    Flush(Vec<u8>),
    FullHouse(u8, u8), // three of a kind, pair
    FourOfAKind(u8, u8), // four of a kind, kicker
    StraightFlush(u8), // highest card (5 for A-2-3-4-5 wheel)
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        use HandRank::*;
        
        // First compare by hand type
        let self_rank = self.hand_type_rank();
        let other_rank = other.hand_type_rank();
        
        match self_rank.cmp(&other_rank) {
            Ordering::Equal => {
                // Same hand type, compare by values
                match (self, other) {
                    (HighCard(a), HighCard(b)) => a.cmp(b),
                    (OnePair(a1, a2), OnePair(b1, b2)) => {
                        a1.cmp(b1).then_with(|| a2.cmp(b2))
                    },
                    (TwoPair(a1, a2, a3), TwoPair(b1, b2, b3)) => {
                        a1.cmp(b1).then_with(|| a2.cmp(b2)).then_with(|| a3.cmp(b3))
                    },
                    (ThreeOfAKind(a1, a2), ThreeOfAKind(b1, b2)) => {
                        a1.cmp(b1).then_with(|| a2.cmp(b2))
                    },
                    (Straight(a), Straight(b)) => a.cmp(b),
                    (Flush(a), Flush(b)) => a.cmp(b),
                    (FullHouse(a1, a2), FullHouse(b1, b2)) => {
                        a1.cmp(b1).then_with(|| a2.cmp(b2))
                    },
                    (FourOfAKind(a1, a2), FourOfAKind(b1, b2)) => {
                        a1.cmp(b1).then_with(|| a2.cmp(b2))
                    },
                    (StraightFlush(a), StraightFlush(b)) => a.cmp(b),
                    _ => Ordering::Equal, // This shouldn't happen
                }
            },
            other => other,
        }
    }
}

impl HandRank {
    fn hand_type_rank(&self) -> u8 {
        match self {
            HandRank::HighCard(_) => 0,
            HandRank::OnePair(_, _) => 1,
            HandRank::TwoPair(_, _, _) => 2,
            HandRank::ThreeOfAKind(_, _) => 3,
            HandRank::Straight(_) => 4,
            HandRank::Flush(_) => 5,
            HandRank::FullHouse(_, _) => 6,
            HandRank::FourOfAKind(_, _) => 7,
            HandRank::StraightFlush(_) => 8,
        }
    }
}

impl Card {
    fn from_str(s: &str) -> Result<Card, &'static str> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 2 {
            return Err("Invalid card format");
        }
        
        let (rank_str, suit_char) = if chars.len() == 2 {
            (chars[0].to_string(), chars[1])
        } else if chars.len() == 3 && s.starts_with("10") {
            ("10".to_string(), chars[2])
        } else {
            return Err("Invalid card format");
        };
        
        let rank = match rank_str.as_str() {
            "2" => 2, "3" => 3, "4" => 4, "5" => 5, "6" => 6, "7" => 7, "8" => 8, "9" => 9,
            "10" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => return Err("Invalid rank"),
        };
        
        let suit = match suit_char {
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            _ => return Err("Invalid suit"),
        };
        
        Ok(Card { rank, suit })
    }
}

fn parse_hand(hand: &str) -> Result<Vec<Card>, &'static str> {
    hand.split_whitespace()
        .map(Card::from_str)
        .collect()
}

fn evaluate_hand(cards: &[Card]) -> HandRank {
    let mut rank_counts = HashMap::new();
    let mut suit_counts = HashMap::new();
    
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
        *suit_counts.entry(card.suit).or_insert(0) += 1;
    }
    
    let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank).collect();
    ranks.sort_by(|a, b| b.cmp(a)); // Sort descending
    
    let is_flush = suit_counts.len() == 1;
    let (is_straight, straight_high) = check_straight(&ranks);
    
    // Check for straight flush
    if is_flush && is_straight {
        return HandRank::StraightFlush(straight_high);
    }
    
    // Group ranks by count
    let mut pairs = Vec::new();
    let mut triples = Vec::new();
    let mut quads = Vec::new();
    let mut singles = Vec::new();
    
    for (&rank, &count) in &rank_counts {
        match count {
            4 => quads.push(rank),
            3 => triples.push(rank),
            2 => pairs.push(rank),
            1 => singles.push(rank),
            _ => {}
        }
    }
    
    // Sort each group in descending order
    pairs.sort_by(|a, b| b.cmp(a));
    triples.sort_by(|a, b| b.cmp(a));
    quads.sort_by(|a, b| b.cmp(a));
    singles.sort_by(|a, b| b.cmp(a));
    
    // Determine hand rank
    if !quads.is_empty() {
        HandRank::FourOfAKind(quads[0], singles[0])
    } else if !triples.is_empty() && !pairs.is_empty() {
        HandRank::FullHouse(triples[0], pairs[0])
    } else if is_flush {
        HandRank::Flush(ranks)
    } else if is_straight {
        HandRank::Straight(straight_high)
    } else if !triples.is_empty() {
        HandRank::ThreeOfAKind(triples[0], singles)
    } else if pairs.len() >= 2 {
        HandRank::TwoPair(pairs[0], pairs[1], singles[0])
    } else if pairs.len() == 1 {
        HandRank::OnePair(pairs[0], singles)
    } else {
        HandRank::HighCard(ranks)
    }
}

fn check_straight(ranks: &[u8]) -> (bool, u8) {
    let mut unique_ranks = ranks.to_vec();
    unique_ranks.sort();
    unique_ranks.dedup();
    
    if unique_ranks.len() != 5 {
        return (false, 0);
    }
    
    // Check for A-2-3-4-5 straight (wheel)
    if unique_ranks == vec![2, 3, 4, 5, 14] {
        return (true, 5); // In wheel straight, 5 is the high card for comparison
    }
    
    // Check for regular straight
    if unique_ranks[4] - unique_ranks[0] == 4 {
        return (true, unique_ranks[4]);
    }
    
    (false, 0)
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return *the same* reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.is_empty() {
        return vec![];
    }
    
    let mut hand_rankings: Vec<(HandRank, &'a str)> = Vec::new();
    
    for &hand in hands {
        if let Ok(cards) = parse_hand(hand) {
            if cards.len() == 5 {
                let rank = evaluate_hand(&cards);
                hand_rankings.push((rank, hand));
            }
        }
    }
    
    if hand_rankings.is_empty() {
        return vec![];
    }
    
    // Sort by hand rank (highest first)
    hand_rankings.sort_by(|a, b| b.0.cmp(&a.0));
    
    // Find all hands with the best rank
    let best_rank = hand_rankings[0].0.clone();
    hand_rankings
        .into_iter()
        .take_while(|(rank, _)| *rank == best_rank)
        .map(|(_, hand)| hand)
        .collect()
}