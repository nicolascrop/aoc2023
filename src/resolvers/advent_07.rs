use std::cmp::Ordering;
use std::collections::HashMap;
use crate::resolvers::Part;

struct Hand {
    raw_cards: String,
    cards: Vec<u32>,
    hand_by_frequency_and_value: Vec<u32>,
    map: HashMap<u32, u32>,
}

struct HandWithBid {
    hand: Hand,
    bid: usize,
}

fn build_hand_map(hand: &Vec<u32>) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for card in hand {
        map.entry(card.clone()).and_modify(|nb_card| *nb_card += 1).or_insert(1);
    }
    return map;
}

fn get_hand(hand: &str, part: Part) -> Hand {
    let cards: Vec<&str> = hand.split("").filter(|value| value.len() > 0).collect();
    let cards_value: Vec<u32> = cards.iter()
        .map(|&value| -> u32 {
            return match value {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => if part == Part::One { 11 } else { 1 },
                "T" => 10,
                _ => value.parse().unwrap(),
            }
        })
        .collect();
    println!("Cards {:?}", cards);
    println!("Cards values {:?}", cards_value);

    assert_eq!(cards_value.len(), 5, "Hand is not of five cards");

    let hand_map = build_hand_map(&cards_value);
    println!("Hand map {:?}", hand_map);

    let mut max_same_cards: u32 = 1;
    let mut hand_sort_by_frequency_and_value: Vec<u32> = Vec::new();
    for hand_map_value in &hand_map {
        hand_sort_by_frequency_and_value.push(*hand_map_value.0);
        if *hand_map_value.1 > max_same_cards {
            max_same_cards = *hand_map_value.1;
        }
    }

    hand_sort_by_frequency_and_value.sort_by(|entry_a, entry_b| -> Ordering {
        let hand_map_value_a = hand_map.get(entry_a).unwrap();
        let hand_map_value_b = hand_map.get(entry_b).unwrap();
        let result_first = hand_map_value_b.cmp(hand_map_value_a);
        if result_first == Ordering::Equal {
            return entry_b.cmp(entry_a);
        }
        return result_first;
    });

    println!("max_same_cards {}", max_same_cards);
    println!("hand_sort_by_frequency_and_value {:?}", hand_sort_by_frequency_and_value);

    return Hand {
        raw_cards: hand.to_string(),
        cards: cards_value,
        hand_by_frequency_and_value: hand_sort_by_frequency_and_value,
        map: hand_map
    };
}

fn get_top_2_values(hand: &HandWithBid, part: Part) -> (u32, u32) {
    let mut top_1 = *hand.hand.map.get(&hand.hand.hand_by_frequency_and_value[0]).unwrap();
    let mut top_2 = hand.hand.map.get(&hand.hand.hand_by_frequency_and_value[0]);
    if part == Part::Two && top_1 != 5 {
        let value_and_frequency_without_jokers: Vec<&u32> = hand.hand.hand_by_frequency_and_value.iter().filter(|v| **v != 1).collect();
        let nb_joker: usize = hand.hand.cards.iter().filter(|v| **v == 1).collect::<Vec<_>>().len();
        top_1 = *hand.hand.map.get(&value_and_frequency_without_jokers[0]).unwrap() + nb_joker as u32;
        if value_and_frequency_without_jokers.len() > 1 {
            top_2 = hand.hand.map.get(&value_and_frequency_without_jokers[1]);
        } else {
            top_2 = None;
        }
    }

    return (
        top_1,
        if top_2.is_some() { *top_2.unwrap() } else { 0 }
    )
}

pub fn resolve(input: &String) {
    let lines = input.lines();
    let mut hands: Vec<HandWithBid> = Vec::new();
    for line in lines {
        println!("line {}", line);
        let parts: Vec<&str> = line.split(" ").collect();
        let hand = get_hand(parts[0].trim(), Part::Two);
        println!("hand: {:?} map {:?}", hand.hand_by_frequency_and_value, hand.map);
        // println!("Score hand: {:?}", score_hand(&parts[0].trim().to_string()));
        hands.push(HandWithBid { hand, bid: parts[1].trim().parse().unwrap()})
    }

    hands.sort_by(|hand_a, hand_b| -> Ordering {
        let hand_a_top_values = get_top_2_values(hand_a, Part::Two);
        let hand_b_top_values = get_top_2_values(hand_b, Part::Two);

        println!("hand_a_top_values {:?}", hand_a_top_values);
        println!("hand_b_top_values {:?}", hand_b_top_values);

        if hand_a_top_values.0 != hand_b_top_values.0 {
            return hand_a_top_values.0.cmp(&hand_b_top_values.0);
        }
        if hand_a_top_values.0 != 5 && hand_a_top_values.1 != hand_b_top_values.1 {
            return hand_a_top_values.1.cmp(&hand_b_top_values.1);
        }

        println!("HERE a {:?} b {:?}", hand_a.hand.raw_cards, hand_b.hand.raw_cards);
        for i in 0..5 {
            if hand_a.hand.cards[i] != hand_b.hand.cards[i] {
                return hand_a.hand.cards[i].cmp(&hand_b.hand.cards[i]);
            }
        }
        return Ordering::Equal;
    });
    println!("Hands size {}", hands.len());
    println!("Sort");

    let mut result = 0;
    for i in 1..=hands.len() {
        let hand = &hands[i - 1];
        println!("Hand {:?}, bid: {}, raw cards {:?}", hand.hand.cards, hand.bid, hand.hand.raw_cards);
        result = result + (i * hand.bid);
    }
    println!("Result {}", result);
}