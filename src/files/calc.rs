// Copyright © 2018 CS410P Robert Horrace
/*
  This is the Rust file with
  the calculator that determines
  the value of a player's hand,
  two plyr_hnd, the flop, the turn,
  and the river. 0 is the high card
  and 9 is the royal flush 
*/

use files::card::*;

/* Calc struct for holding total hand */
#[derive(Clone,Copy)]
pub struct Calc {
  plyr_hnd: [Card; 7],
  hand_sz: usize,
  card_cnt: [i64; 15],
  suit_cnt: [i64; 5],
}

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

  /* Check if hand contains a certain card */
  fn contains(self,val: i64, st: i64) -> bool {
    self.plyr_hnd.iter()
                 .any(|&x| x.value() == val && x.suit() == st)
  }

  /* Get maximum suit , will only be used when there is a flush */
  pub fn max_suit(self) -> i64 {
    let mut i = 0;
    for (j, &val) in self.suit_cnt.iter()
                                  .skip(1)
                                  .enumerate() {
      if val > self.suit_cnt[i] {
        i = j;
      }
    }
    i as i64
  }
  
  /* Standard insertion sort */
  fn sort(&mut self) {
    for i in 1..self.hand_sz {
      let key = self.plyr_hnd[i];
      let mut j = i-1;
      while j > 0 && self.plyr_hnd[j].value() > key.value() {
        self.plyr_hnd[j+1] = self.plyr_hnd[j];
        j -= 1;
      }
      self.plyr_hnd[j+1] = key;
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
    let len = self.card_cnt.iter()
                           .skip(1)
                           .filter(|x| **x > 0)
                           .count();
    if len < 5 {
      return false;
    }
    let strght = self.card_cnt.iter()
                              .skip(1)
                              .enumerate()
                              .filter(|(_,x)| **x > 0);
    let mut prev = 0;
    let mut count = 1;
    for (i,_) in strght {
      if i == prev + 1 {
        count += 1;
      }
      else {
        if count >= 5 {
          break;
        }
        count = 1;
      }
      prev = i;
    }
    count >= 5
  }

  /* Check for straight flush */
  fn straight_flush(self,st: i64) -> bool {
    let mut prev = 0;
    let mut count = 0;
    if self.plyr_hnd.iter()
                    .any(|&x| x.value() == 14 && x.suit() == st) {
      prev = 1;
      count = 1;
    }
    for crd in self.plyr_hnd.iter()
                         .filter(|x| x.suit() == st) {
      if crd.value() == prev + 1 {
        count += 1;
        prev += 1;
      }
      else {
        prev = crd.value();
        count = 1;
      }
    }
    if count >= 5 {
      return true;
    }
    false
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
          let mut have = false;
          for i in 10..15 {              // Check royal flush
            have = self.contains(i as i64,suit);
            if !have {
              break;
            }
          }
          match have {                    // If has allplyr_hnd for RF
            true  => val = 9,             // Royal Flush
            false => val = 8,             // Straight Flush
          }
        }
      }
      else {
        match fours {                     // If Four of a Kind
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
  

} 
