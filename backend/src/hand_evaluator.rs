use crate::cards::{Card, Rank};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandRank {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

pub fn evaluate_hand(cards: &[Card]) -> HandRank {
    // Expecting at least 5 cards. The logic chooses the best 5-card combination from the given cards.
    // It's assumed that the caller won't call this with fewer than 5 cards. If less than 5, handle gracefully:
    if cards.len() < 5 {
        // Fallback if called prematurely:
        let highest_rank = cards.iter().map(|c| c.rank).max().unwrap_or(Rank::Two);
        return HandRank::HighCard(highest_rank);
    }

    let best_rank = best_five_card_rank(cards);
    best_rank
}

fn best_five_card_rank(cards: &[Card]) -> HandRank {
    // Generate all 5-card combinations and evaluate each, returning the best
    let combos = get_five_card_combinations(cards);
    combos.into_iter().map(|c| evaluate_five_card_hand(&c)).max().unwrap()
}

fn get_five_card_combinations(cards: &[Card]) -> Vec<Vec<Card>> {
    let mut combinations = Vec::new();
    let card_count = cards.len();
    let indices: Vec<usize> = (0..card_count).collect();

    for i1 in 0..(card_count - 4) {
        for i2 in (i1 + 1)..(card_count - 3) {
            for i3 in (i2 + 1)..(card_count - 2) {
                for i4 in (i3 + 1)..(card_count - 1) {
                    for i5 in (i4 + 1)..card_count {
                        let combo = vec![
                            cards[indices[i1]],
                            cards[indices[i2]],
                            cards[indices[i3]],
                            cards[indices[i4]],
                            cards[indices[i5]],
                        ];
                        combinations.push(combo);
                    }
                }
            }
        }
    }

    if card_count == 5 {
        combinations.push(cards.to_vec());
    }

    combinations
}

fn evaluate_five_card_hand(cards: &[Card]) -> HandRank {
    let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
    ranks.sort_by(|a, b| b.cmp(a)); // descending order

    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);
    let is_straight = is_consecutive(&ranks);

    let rank_counts = get_rank_counts(&ranks);

    match (is_flush, is_straight, &ranks[..]) {
        (true, true, [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, ..]) => HandRank::RoyalFlush,
        (true, true, _) => HandRank::StraightFlush(ranks[0]),
        _ if has_four_of_a_kind(&rank_counts) => {
            let quad = get_n_of_a_kind_rank(&rank_counts, 4);
            HandRank::FourOfAKind(quad)
        }
        _ if has_full_house(&rank_counts) => {
            let trip = get_n_of_a_kind_rank(&rank_counts, 3);
            let pair = get_n_of_a_kind_rank(&rank_counts, 2);
            HandRank::FullHouse(trip, pair)
        }
        (true, false, _) => HandRank::Flush(ranks[0]),
        (false, true, _) => HandRank::Straight(ranks[0]),
        _ if has_three_of_a_kind(&rank_counts) => {
            let trip = get_n_of_a_kind_rank(&rank_counts, 3);
            HandRank::ThreeOfAKind(trip)
        }
        _ if has_two_pair(&rank_counts) => {
            let high_pair = get_high_pair_rank(&rank_counts);
            let low_pair = get_low_pair_rank(&rank_counts);
            HandRank::TwoPair(high_pair, low_pair)
        }
        _ if has_one_pair(&rank_counts) => {
            let pair = get_n_of_a_kind_rank(&rank_counts, 2);
            HandRank::OnePair(pair)
        }
        _ => HandRank::HighCard(ranks[0]),
    }
}

fn is_consecutive(ranks: &[Rank]) -> bool {
    let mut sorted = ranks.to_vec();
    sorted.sort();

    // Check wheel (A-2-3-4-5)
    let wheel = vec![Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two];
    let sorted_wheel = {
        let mut w = wheel.clone();
        w.sort();
        w
    };
    if sorted == sorted_wheel {
        return true;
    }

    for i in 0..(sorted.len() - 1) {
        if (sorted[i + 1] as u8) != (sorted[i] as u8 + 1) {
            return false;
        }
    }
    true
}

fn get_rank_counts(ranks: &[Rank]) -> HashMap<Rank, usize> {
    let mut counts = HashMap::new();
    for &r in ranks {
        *counts.entry(r).or_insert(0) += 1;
    }
    counts
}

fn has_four_of_a_kind(counts: &HashMap<Rank, usize>) -> bool {
    counts.values().any(|&c| c == 4)
}

fn has_full_house(counts: &HashMap<Rank, usize>) -> bool {
    counts.values().any(|&c| c == 3) && counts.values().any(|&c| c == 2)
}

fn has_three_of_a_kind(counts: &HashMap<Rank, usize>) -> bool {
    counts.values().any(|&c| c == 3)
}

fn has_two_pair(counts: &HashMap<Rank, usize>) -> bool {
    counts.values().filter(|&&c| c == 2).count() == 2
}

fn has_one_pair(counts: &HashMap<Rank, usize>) -> bool {
    counts.values().any(|&c| c == 2)
}

fn get_n_of_a_kind_rank(counts: &HashMap<Rank, usize>, n: usize) -> Rank {
    counts.iter().filter(|(_, &c)| c == n).map(|(&r, _)| r).max().unwrap()
}

fn get_high_pair_rank(counts: &HashMap<Rank, usize>) -> Rank {
    counts.iter().filter(|(_, &c)| c == 2).map(|(&r, _)| r).max().unwrap()
}

fn get_low_pair_rank(counts: &HashMap<Rank, usize>) -> Rank {
    counts.iter().filter(|(_, &c)| c == 2).map(|(&r, _)| r).min().unwrap()
}
