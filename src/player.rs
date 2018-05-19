// Copyright © 2018 CS410P Robert Horrace
/*
  This is the Rust file
  containg the player and
  its functions. It will
  receive two cards and give 
  them to calc
*/

use card::*;

#[derive(Clone,Copy)]
pub struct Player {
  hand: [Card; 2],
  combo: u64, /* 0 for high card, 9 for Royal flush */
}

impl Player {
  /* Initialize new player */
  pub fn new() -> Player {
    Player { hand: [Card::new(); 2],
             combo: 0 }
  }
  
  /* Clean hand */
  pub fn clear(&mut self) {
    self.hand = [Card::new(); 2];
    self.combo = 0;
  }

  pub fn _get_val(self) -> u64 {
    self.combo
  }

  /* get combo val in str */
  pub fn get_combo(self) -> &'static str {
    match self.combo {
      1 => "One Pair",
      2 => "Two Pair",
      3 => "Three of a Kind",
      4 => "Straight",
      5 => "Flush",
      6 => "Full House",
      7 => "Four of a Kind",
      8 => "Straight Flush",
      9 => "Rpyal Flush",
      _ => "High Card",
    }
  } 

  /* receive the hand */
  pub fn rcv_hand(&mut self,hnd: [Card; 2]) {
    self.hand = hnd;
  }

  /* Updates player hand value before the flop */
  pub fn update(&mut self) {
    match self.hand[0].value() == self.hand[1].value() {
      true  => self.combo = 1,
      false => self.combo = 0,
    }
  }
  
  /* Update after fold */
  pub fn update_fold(&mut self,fld_combo: u64) {
    if self.combo != fld_combo {
      self.combo = fld_combo;
    }
  }

  /* Update after turn */
  pub fn update_turn(&mut self,trn_combo: u64) {
    if self.combo != trn_combo {
      self.combo = trn_combo;
    }
  }

  /* Update after river */
  pub fn update_river(&mut self,rvr_combo: u64) {
    if self.combo != rvr_combo {
      self.combo = rvr_combo;  
    }
  }

  pub fn display(self) {
    print!("Hand: ");
    for crd in self.hand.iter() {
      crd.display();
      if !crd.is(&self.hand[1]) {
        print!(" | ");
      }
    }
    println!();
  }
}
