// Copyright © 2018 CS410P Robert Horrace
/*
  This is the tie file that
  contains functions that
  determine the winners
  of a tie breaker.
*/

use game::card::*;

pub fn high_card(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                    // Highest 5 cards
             .enumerate()
             .skip(2)
             .filter(|(_,x)| **x == 1)
             .rev()
             .take(5);
  let p = plyr.iter()                   // Highest 5 cards
              .enumerate()
              .skip(2)
              .filter(|(_,x)| **x == 1)
              .rev()
              .take(5);
  for ((i,_),(j,_)) in c.zip(p) {
    if i > j {                          // If computer has higher value
      return -1;
    }
    else if i < j {                     // If player has higher value
      return 1;
    }
  }
  0                                     // Tie
}

/* Functions to break tie with one pair */
pub fn one_pair(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                    // Single pair
             .enumerate()
             .skip(2)
             .filter(|(_,x)| **x == 2);
  let p = plyr.iter()                   // Single pair
              .enumerate()
              .skip(2)
              .filter(|(_,x)| **x == 2);
  for ((i,_),(j,_)) in c.zip(p) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  /* Tie, check kickers */
  let c_kick = com.iter()               // Kickers (highest 3 cards)
                  .enumerate()
                  .skip(2)
                  .filter(|(_,x)| **x == 1)
                  .rev()
                  .take(3);
  let p_kick = plyr.iter()              // Kickers (highest 3 cards)
                   .enumerate()
                   .skip(2)
                   .filter(|(_,x)| **x == 1)
                   .rev()
                   .take(3);
  for ((i,_),(j,_)) in c_kick.zip(p_kick) {
    if i > j {                          // Computer wins
      return -1;
    }
    else if i < j {                     // Player wins
      return 1;
    }
  }
  0                                     // Tie
}

pub fn two_pair(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                // Highest two pairs
             .enumerate()
             .skip(2)
             .filter(|(_,x)| **x == 2)
             .rev()
             .take(2);
  let p = plyr.iter()               // Highest two pairs
              .enumerate()
              .skip(2)
              .filter(|(_,x)| **x == 2)
              .rev()
              .take(2);
  let mut c_prs = [0; 2];
  let mut p_prs = [0; 2];
  let mut c_sz = 0;
  let mut p_sz = 0;
  for ((i,_),(j,_)) in c.zip(p) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
    c_prs[c_sz] = i;                    // Collect used pairs
    p_prs[p_sz] = j;                    // Collect used pairs
    c_sz += 1;
    p_sz += 1;
  }
  /* Tie, check kickers */
  let c_kick = com.iter()               // Kicker (highest card)
                  .enumerate()
                  .skip(2)
                  .filter(|(x,y)| **y > 0 && !c_prs.iter().any(|z| *x == *z))
                  .rev()
                  .take(1);
  let p_kick = plyr.iter()              // Kicker (highest card)
                   .enumerate()
                   .skip(2)
                   .filter(|(x,y)| **y > 0 && !p_prs.iter().any(|z| *x == *z))
                   .rev()
                   .take(1);
  for ((i,_),(j,_)) in c_kick.zip(p_kick) {
    if i > j {                          // Computer wins
      return -1;
    }
    else if i < j {                     // Player wins
      return 1;
    }
  }
  0                                     //Tie
}
/* Function to break tie on three of kind or*/
pub fn three_of_kind(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                    // Filter out cards that don't have 3 count
             .enumerate()
             .skip(2)
             .filter(|(_,x)| **x == 3)
             .rev();
  let p = plyr.iter()                   // Filter out cards that don't have 3 count
              .enumerate()
              .skip(2)
              .filter(|(_,x)| **x == 3)
              .rev();
  for ((i,_),(j,_)) in c.zip(p) {
    if i > j {
      return -1;                        // If computer wins
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  /* Tie */
  let c_kick = com.iter()               // Kickers (highest two cards)
                  .enumerate()
                  .skip(2)
                  .filter(|(_,x)| **x == 1)
                  .rev()
                  .take(2);
  let p_kick = plyr.iter()              // Kickers (highest two cards)
                   .enumerate()
                   .skip(2)
                   .filter(|(_,x)| **x == 1)
                   .rev()
                   .take(2);
  for ((i,_),(j,_)) in c_kick.zip(p_kick) {
    if i > j {
      return -1;                        // If computer wins
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  0                                     // Tie
}

/* Function to break tie on straight */
pub fn straight(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                    // Prepare for getting straights
             .enumerate()
             .skip(1)
             .filter(|(_,x)| **x > 0)
             .rev();
  let p = plyr.iter()                   // Prepare form getting straights
              .enumerate()
              .skip(1)
              .filter(|(_,x)| **x > 0)
              .rev();
  let mut c_cnt = 0;
  let mut p_cnt = 0;
  let mut c_strght = [0; 5];
  let mut p_strght = [0; 5];
  for ((i,_),(j,_)) in c.zip(p) {       // Get straights
    if c_cnt != 5 {                     // Get computer's straight
      if c_cnt == 0 {
        c_strght[c_cnt] = i;
        c_cnt += 1;
      }
      else if i == c_strght[c_cnt-1] - 1 {
        c_strght[c_cnt] = i;
        c_cnt += 1;
      }
      else {
        c_cnt = 0;
        c_strght[c_cnt] = i;
        c_cnt += 1;
      }
    }
    if p_cnt != 5 {                     // Get player's straight
      if p_cnt == 0 {
        p_strght[p_cnt] = j;
        p_cnt += 1
      }
      else if j == p_strght[p_cnt-1] - 1 {
        p_strght[p_cnt] = j;
        p_cnt += 1;
      }
      else {
        p_cnt = 0;
        p_strght[p_cnt] = j;
        p_cnt += 1;
      }
    }
    if c_cnt == 5 && p_cnt == 5 {       // When straights have been gotten
      break;
    }
  }
  for (i,j) in c_strght.iter()
                       .zip(p_strght.iter()) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  0                                     // Tie
}

pub fn flush(com: [Card; 7],c_suit: i64,plyr: [Card; 7],p_suit: i64) -> i64 {
  let c = com.iter()                    // Get highest five cards with same suit
             .filter(|x| x.suit() == c_suit)
             .rev()
             .take(5);
  let p = plyr.iter()                   // Get highest five cards with same suit
              .filter(|x| x.suit() == p_suit)
              .rev()
              .take(5);
  for (i,j) in c.zip(p) {
    if i.value() > j.value() {          // If computer wins
      return -1;
    }
    else if i.value() < j.value() {     // If player wins
      return 1;
    }
  }
  0                                     // Tie
}

pub fn full_house(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                    // Filter cards to take com's highest three of kind
             .enumerate()
             .skip(2)
             .filter(|(_,x)| **x == 3)
             .rev()
             .take(1);
  let p = plyr.iter()                   // Filter cards to take plyr's highest three of kind
              .enumerate()
              .skip(2)
              .filter(|(_,x)| **x == 3)
              .rev()
              .take(1);
  let mut c_trip = 0;
  let mut p_trip = 0;
  for ((i,_),(j,_)) in c.zip(p) {       // Will only execute once, no loop
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
    c_trip = i;                         // Get value
    p_trip = j;                         // Get value
  }        
  let c_p = com.iter()                  // Get the next highest pair/three of a kind
               .enumerate()
               .skip(2)
               .filter(|(x,y)| (**y == 2 || **y == 3) && *x != c_trip)
               .rev()
               .take(1);
  let p_p = plyr.iter()                 // Get the next highest pair/three of a kind
                .enumerate()
                .skip(2)
                .filter(|(x,y)| (**y == 2 || **y == 3) && *x != p_trip)
                .rev()
                .take(1);
  for ((i,_),(j,_)) in c_p.zip(p_p) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  0                                     // Tie
}

pub fn four_of_kind(com: [i64; 15],plyr: [i64; 15]) -> i64 {
  let c = com.iter()                    // Filter cards that don't have 4 count
             .enumerate()
             .skip(2)
             .filter(|(_,x)| **x == 4);
  let p = plyr.iter()                   // Filter cards that don't have 4 count
              .enumerate()
              .skip(2)
              .filter(|(_,x)| **x == 4);
  for ((i,_),(j,_)) in c.zip(p) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  /* Tie */
  let c_kick = com.iter()               // Kicker (next highest card)
                  .enumerate()
                  .skip(2)
                  .filter(|(_,x)| **x < 4 && **x > 0)
                  .rev()
                  .take(1);
  let p_kick = plyr.iter()               // Kicker (next highest card)
                   .enumerate()
                   .skip(2)
                   .filter(|(_,x)| **x < 4 && **x > 0)
                   .rev()
                   .take(1);
  for ((i,_),(j,_)) in c_kick.zip(p_kick) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  0                                     // Tie
}

pub fn straight_flush(com: [Card; 7],c_suit: i64,plyr: [Card; 7],p_suit: i64) -> i64 {
  let c = com.iter()                    // Filter cards not of computer's flush suit
             .filter(|x| x.suit() == c_suit)
             .rev();
  let p = plyr.iter()                   // Filter cards not of player's flush suit
              .filter(|x| x.suit() == p_suit)
              .rev();
  let mut c_strght = [0; 5];
  let mut p_strght = [0; 5];
  let mut c_size = 0;
  let mut p_size = 0;
  for (i,j) in c.zip(p) {               // Get straights from both hands
    if c_size != 5 {                    // From computer's hand
      if c_size == 0 {
        c_strght[c_size] = i.value();
        c_size += 1;
      }
      else if i.value() == c_strght[c_size-1] - 1 {
        c_strght[c_size] = i.value();
        c_size += 1;
        if i.value() == 2 && c_size == 4{
            c_strght[c_size] = 1;
            c_size += 1;
        }
      }
      else {
        c_size = 0;
        c_strght[c_size] = i.value();
        c_size += 1;
      }
    }
    if p_size != 5 {                    // From player's hand
      if p_size == 0 {
        p_strght[p_size] = j.value();
        p_size += 1;
      }
      else if j.value() == p_strght[c_size-1] - 1 {
        p_strght[c_size] = j.value();
        p_size += 1;
        if j.value() == 2 && p_size == 4{
            p_strght[p_size] = 1;
            p_size += 1;
        }
      }
      else {
        p_size = 0;
        p_strght[p_size] = j.value();
        p_size += 1;
      }
    }
    
    if c_size == 5 && p_size == 5 {     // When hands have been gotten
      break;
    }
  }
  for (i,j) in c_strght.iter()          // Compare hands
                       .zip(p_strght.iter()) {
    if i > j {                          // If computer wins
      return -1;
    }
    else if i < j {                     // If player wins
      return 1;
    }
  }
  0                                     // Tie
} 
