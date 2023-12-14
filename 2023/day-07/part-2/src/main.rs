use std::cmp::Ordering;
use std::fs;

#[derive(Clone)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: char,
}

fn get_number_cards_repeated(cards: &str, card: char) -> u8 {
    cards
    .chars()
    .enumerate()
    .filter(|(_, c)| *c == card)
    .map(|(i, _)| i)
    .collect::<Vec<_>>().len().try_into().unwrap()
}

fn calculate_hand_types(hands: &Vec<Hand>) -> Vec<char> {
    // hand_types
    // A: Five of a kind
    // B: Four of a kind
    // C: Full house
    // D: Three of a kind
    // E: Two pair
    // F: One pair
    // G: High card

    let mut hand_types: Vec<char> = vec![];
    let cards = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    for hand in hands {
        let mut cards_repeated: Vec<u8> = vec![];
        for card in cards.iter() {
            cards_repeated.push(get_number_cards_repeated(&hand.cards, *card));
        }        

        // Check repeated array to rank the card
        let mut hand_type;
        if cards_repeated.iter().filter(|&n| *n == 5).count() == 1 {
            hand_type = 'A'; // Five of a kind
        } else if cards_repeated.iter().filter(|&n| *n == 4).count() == 1 {
            hand_type = 'B'; // Four of a kind
        } else if cards_repeated.iter().filter(|&n| *n == 3).count() == 1 {
            if cards_repeated.iter().filter(|&n| *n == 2).count() == 1 {
                hand_type = 'C'; // Full house
            } else {
                hand_type = 'D'; // Three of a kind
            }
        } else if cards_repeated.iter().filter(|&n| *n == 2).count() == 2 {
            hand_type = 'E'; // Two pair
        } else if cards_repeated.iter().filter(|&n| *n == 2).count() == 1 {
            hand_type = 'F'; // One pair
        } else {
            hand_type = 'G'; // High card
        }

        // Check jokers
        let number_of_jokers = cards_repeated[3];
        // Works in every case except two pair
        if number_of_jokers == 1 || number_of_jokers == 2 {
            // println!("Before jokers: {}", hand_type);
            if hand_type == 'D' || hand_type == 'E' {
                hand_type = (hand_type.to_string().as_bytes()[0] - number_of_jokers - 1) as char;
            } else if hand_type == 'F' {
                hand_type = 'D';    
            } 
            else {
                hand_type = (hand_type.to_string().as_bytes()[0] - number_of_jokers) as char;
            }
            // println!("After jokers: {}", hand_type);
        } else if number_of_jokers == 3 {
            if hand_type == 'C' {
                hand_type = 'A';
            } else if hand_type == 'D' {
                hand_type = 'B';
            }
        } else if number_of_jokers == 4 {
            hand_type = 'A';
        }

        hand_types.push(hand_type);
    }

    return hand_types;
}

fn calculate_winnings(mut hands: Vec<Hand>) -> u32 {
    let mut winnings: u32 = 0;

    hands.sort_by(|a, b| { 
        if a.hand_type > b.hand_type {
            return Ordering::Less;
        } else if a.hand_type == b.hand_type {
            let cards = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
            // println!("Cards with same rank: {} - {}", a.cards, b.cards);
            let mut index_card = 0;
            while index_card < 5 {
                let a_card_index = cards.iter().position(|&x| x == a.cards.chars().nth(index_card).unwrap());
                let b_card_index = cards.iter().position(|&x| x == b.cards.chars().nth(index_card).unwrap());
                if a_card_index < b_card_index {
                    return Ordering::Greater;
                } else if a_card_index > b_card_index {
                    return Ordering::Less;        
                }
                index_card += 1;
            }
            return Ordering::Greater;
        } 
        return Ordering::Greater;
    });

    let mut index_hand = 0;
    while index_hand < hands.len() {
        let u32_index_hand: u32 = index_hand.try_into().unwrap();
        winnings += (u32_index_hand + 1) * hands[index_hand].bid;
        println!("Hand: {}, Type: {}, Rank: {}, Winnings: {}", 
                 hands[index_hand].cards, 
                 hands[index_hand].hand_type,
                 index_hand + 1, 
                 winnings);
        index_hand += 1;
    }

    return winnings;
}

fn main() {
    let total_winnings;

    let mut hands: Vec<Hand> = vec![];
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        hands.push(Hand {
            cards: line.split(" ").into_iter().collect::<Vec<_>>()[0].to_string(),
            bid: line.split(" ").into_iter().collect::<Vec<_>>()[1].parse::<u32>().unwrap(),
            hand_type: ' ',
        });
    }   

    let hand_types = calculate_hand_types(&hands);
    // println!("Ranks: {:?}", hand_types);
    
    let mut cloned_hands = hands.clone();
    let mut index_hand = 0;
    while index_hand < cloned_hands.len() {
        cloned_hands[index_hand].hand_type = hand_types[index_hand];
        index_hand += 1;
    }
    
    total_winnings = calculate_winnings(cloned_hands);
    
    println!("Total winnings: {}", total_winnings);
 }
