// Copyright © 2018 CS410P Robert Horrace
/*
  This is the Rust file with
  the calculator that determines
  the value of a player's hand,
  two plyr_hnd, the flop, the turn,
  and the river. 0 is the high card
  and 9 is the royal flush 
*/

use game::card::*;

/* Calc struct for holding total hand */
#[derive(Clone,Copy)]
pub struct Calc {
  plyr_hnd: [Card; 7],  // Player total hand
  hand_sz: usize,       // Amount of cards in hand
  card_cnt: [i64; 15],  // Count of card values
  suit_cnt: [i64; 5],   // Count of card suits
}

/* functions for Calc */
impl Calc {
  /* New function */
  pub fn new() -> Calc {
    Calc{ plyr_hnd: [Card::new(); 7],
          hand_sz: 0,
          card_cnt: [0; 15],
          suit_cnt: [0; 5] }
  }

  /* Get hand function */
  pub fn get_hnd(self) -> [Card; 7] {
    self.plyr_hnd
  }

  /* Get card count function */
  pub fn get_card_cnt(self) -> [i64; 15] {
    self.card_cnt
  }

  /* Clear the hand, set size to 0, so on and so forth */
  pub fn clear(&mut self) {
    self.plyr_hnd = [Card::new(); 7];
    self.hand_sz = 0;
    self.card_cnt = [0; 15];
    self.suit_cnt = [0; 5];
  }

  /* Get maximum suit , will only be used when there is a flush */
  fn max_suit(self) -> i64 {
    let mut i = 0;
    for (j, &val) in self.suit_cnt.iter()
                                  .enumerate()
                                  .skip(1) {
      if val > self.suit_cnt[i] {
        i = j;
      }
    }
    i as i64
  }
  
  /* Standard insertion sort */
  fn sort(&mut self) {
    for i in 1..self.hand_sz {
      for j in (1..i+1).rev() {
        if self.plyr_hnd[j-1].value() <= self.plyr_hnd[j].value() {
          break;
        }
        self.plyr_hnd.swap(j-1,j);
      }
    }  
  }

  /* Add player's hand to Calc */
  pub fn add_hand(&mut self, hand: [Card; 2]) {
    for card in hand.iter() {
      let crd_val = card.value() as usize;
      let st_val = card.suit() as usize;
      self.plyr_hnd[self.hand_sz] = *card;
      self.hand_sz += 1;
      self.suit_cnt[st_val] += 1;
      self.card_cnt[crd_val] += 1;
      if crd_val == 14 {
        self.card_cnt[1] += 1;
      }
    }
    self.sort();
  }

  /* Add flop to Calc */
  pub fn add_cards(&mut self, flop: [Card; 3]) {
    for card in flop.iter() {
      let crd_val = card.value() as usize;
      let st_val = card.suit() as usize;
      self.plyr_hnd[self.hand_sz] = *card;
      self.hand_sz += 1;
      self.suit_cnt[st_val] += 1;
      self.card_cnt[crd_val] += 1;
      if crd_val == 14 {
        self.card_cnt[1] += 1;
      }
    }
    self.sort();
  }

  /* Add turn/river to Calc */
  pub fn add(&mut self, turn: Card) {
    let crd_val: usize = turn.value() as usize;
    let st_val: usize = turn.suit() as usize;
    self.plyr_hnd[self.hand_sz] = turn;
    self.hand_sz += 1;
    self.suit_cnt[st_val] += 1;
    self.card_cnt[crd_val] += 1;
    if crd_val == 14 {
      self.card_cnt[1] += 1;
    }
    self.sort();
  }
  
  /* Check specific suit count */
  fn contains_st_cnt(self, cnt: i64) -> bool {
    self.suit_cnt.iter()
                 .skip(1)
                 .any(|&x| x >= cnt)
  }

  /* Count specific card amount */
  fn cnt(self, val: i64) -> i64 {
    self.card_cnt.iter()
                 .skip(2)
                 .filter(|x| **x == val)
                 .count() as i64
  }

  /* Check if a straight is in hand */
  fn check_strght(self) -> bool {
    let mut count = 0;
    let mut prev = 1;
    for card in self.plyr_hnd.iter()
                             .rev() {
      if card.value() == prev - 1 {
        count += 1;
        if count == 5 {
          return true;
        }
      }
      else if card.value() == prev {
        continue;
      }
      else {
        count = 1;
      }
      prev = card.value();
    }
    if prev == 2 && count == 4 {
      return self.plyr_hnd.iter()
                          .any(|x| x.value() == 14);
    }
    false
  }

  /* Check for straight flush */
  fn straight_flush(self,st: i64) -> bool {
    let mut prev = 0;
    let mut count = 0;
    for card in self.plyr_hnd.iter()
                             .rev()
                             .filter(|x| x.suit() == st) {
      if card.value() == prev - 1 {
        count += 1;
        if count == 5 {
          return true;
        }
      }
      else {
        count = 1;
      }
      prev = card.value();
    }
    if prev == 2 && count == 4 {
      return self.plyr_hnd.iter()
                          .any(|x| x.value() == 14 && x.suit() == st);
    }
    false
  }

  fn royal_flush(self,st: i64) -> bool {
    let mut flush = self.plyr_hnd.iter()
                                 .rev()
                                 .filter(|x| x.suit() == st)
                                 .take(5);
    let mut ace = Card::new();
    ace.change(14,st);
    let mut ten = Card::new();
    ten.change(10,st);
    flush.nth(0) == Some(&ace) && flush.last() == Some(&ten)
    
  }

  /* Calculate hand value */
  pub fn calc_hand(self) -> u64 {
    let mut val: u64 = 0;
    let pairs = self.cnt(2);             // Count pairs
    let threes = self.cnt(3);            // Count three of kinds
    let fours = self.cnt(4);             // Count four of kinds
    let flush = self.contains_st_cnt(5); // Check flush
    let straight = self.check_strght();  // Check straight
    if flush {                           // If flush
      let suit = self.max_suit();
      if straight {                      // If straight
        if self.straight_flush(suit) {   // If straight flush
          match self.royal_flush(suit) { // If has all plyr_hnd for RF
            true  => val = 9,            // Royal Flush
            false => val = 8,            // Straight Flush
          }
        }
      }
      else {
        match fours {                     // If four of a Kind
          1 => val = 7,                   // Four of a kind
          _ => val = 5,                   // Flush
        }
      }
    }
    else if straight {                    // If straight
      val = 4;
    }
    else if fours == 1 {                  // If four of a kind
      val = 7;
    }
    else if threes >= 1 || pairs >= 1 {   // If pairs or threes of kind
      match (threes,pairs) {
        (0,1) => val = 1,                 // One pair
        (0,_) => val = 2,                 // Two pair
        (1,0) => val = 3,                 // Three of a kind
        (_,_) => val = 6,                 // Full house
      }
    }
    val                                   // High card if all flags fail
  } 
 
  /* Get best hand from the 7 cards */ 
  pub fn best_hand(self,hand_val: u64) -> [Card; 5] {
    let mut hand = [Card::new(); 5];
    let mut i = 0;
    match hand_val {
    9 =>  // Royal Flush
      {
        let suit = self.max_suit();
        for card in self.plyr_hnd.iter()
                                 .filter(|x| x.suit() == suit)
                                 .rev()
                                 .take(5) {
          hand[i] = *card;
          i += 1;
        }
      },
    8 =>  // Straight Flush
      {
        let suit = self.max_suit();
        let mut i = 0;
        for card in self.plyr_hnd.iter()
                                 .filter(|x| x.suit() == suit)
                                 .rev() {
          if i == 0{
            hand[i] = *card;
            i += 1;
          }
          else if card.value() == hand[i-1].value() - 1 {
            hand[i] = *card;
            i += 1;
            if i == 5 {
              break;
            }
          }
          else {
            i = 0;
            hand[i] = *card;
            i += 1;
          }
        }
        if i == 4 {
          hand[i].change(1,suit);
        }
      },
    7 =>  // Four of a kind
      {
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 4)
                                 .take(4) {
          hand[i] = *card;
          i += 1;
        }
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] < 4)
                                 .rev()
                                 .take(1) {
          hand[i] = *card;
          i += 1;
        }
      },
    6 =>  // Full house
      {
        let mut used = [0; 1];
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 3)
                                 .rev()
                                 .take(3) {
          hand[i] = *card;
          i += 1;
          used[0] = card.value();
        }
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] > 1 &&
                                             x.value() != used[0])
                                 .rev()
                                 .take(2) {
          hand[i] = *card;
          i += 1;
        }
      },
    5 =>  // Flush
      {
        let suit = self.max_suit();
        for card in self.plyr_hnd.iter()
                                 .filter(|x| x.suit() == suit)
                                 .rev()
                                 .take(5) {
          hand[i] = *card;
          i += 1;
        }
      },
    4 =>  // Straight
      {
        for card in self.plyr_hnd.iter()
                                 .rev() {
          if i == 0 {
            hand[i] = *card;
            i += 1;
          }
          else if card.value() == hand[i-1].value() - 1 {
            hand[i] = *card;
            i += 1;
            if i == 5 {
              break;
            }
          }
          else if card.value() == hand[i-1].value() {
            continue;
          }
          else {
            i = 0;
            hand[i] = *card;
            i += 1;
          }
        }
        if i == 4 {
          for card in self.plyr_hnd.iter()
                                   .filter(|x| x.value() == 14) 
                                   .take(1) {
            hand[i] = *card;
            hand[i].change(1,card.suit());
            i += 1;
          }
        }
      },
    3 =>  // Three of a kind
      {
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 3)
                                 .rev() {
          hand[i] = *card;
          i += 1;
        }
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 1)
                                 .rev()
                                 .take(2) {
          hand[i] = *card;
          i += 1;
        }
      },
    2 =>  // Two pair
      {
        let mut used = [0; 4];
        let mut j = 0;
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 2)
                                 .rev()
                                 .take(4) {
          hand[i] = *card;
          i += 1;
          used[j] = card.value();
          j += 1;
        }
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] > 0 &&
                                             !used.iter().any(|&y| y == x.value()))
                                 .rev()
                                 .take(1) {
          hand[i] = *card;
          i += 1;
        }
      },
    1 =>  // One pair 
      {
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 2) {
          hand[i] = *card;
          i += 1;
        }
        for card in self.plyr_hnd.iter()
                                 .filter(|x| self.card_cnt[x.value() as usize] == 1)
                                 .rev()
                                 .take(3) {
          hand[i] = *card;
          i += 1;
        }
      },
    _ =>  // High card
      {
        for card in self.plyr_hnd.iter()
                                 .rev()
                                 .take(5) {
          hand[i] = *card;
          i += 1;
        }
      },
    }
    hand  // return hand
  }
} 
