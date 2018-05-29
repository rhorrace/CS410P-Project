// Copyright © 2018 CS410P Robert Horrace
/*
  This file hold the computer the 
  player is playing against. It will calc  
  its own hand value and will update on its 
  own.
*/

use card::*;
use player::*;
use calc::*;

/* Com struct, holding hand and option value */
#[derive(Clone,Copy)]
pub struct Com {
  hand: Player,
  calc: Calc,
}

/* Impl for Com struct */
impl Com {
  
  /* New function */
  pub fn new() -> Com {
    Com { hand: Player::new(),
          calc: Calc::new(), }
  }

  pub fn get_val(self) -> u64 {
    self.hand.get_val()
  }

  /* Get hand function */
  pub fn get_hnd(self) -> [Card; 7] {
    self.calc.get_hnd()
  }

  /* Get card count function */
  pub fn get_card_cnt(self) -> [i64; 15] {
    self.calc.get_card_cnt()
  }
  
  /* Get maximum suit in hand,only when there is flush */
  pub fn max_suit(self) -> i64 {
    self.calc.max_suit()
  } 
  
  /* Clear function, cleans up variables for next game*/
  pub fn clear(&mut self) {
    self.hand.clear();
    self.calc.clear();
  }

  /* Receive hand from dealer */
  pub fn rcv_hand(&mut self,hnd: [Card; 2]) {
    self.hand.rcv_hand(hnd);
    self.calc.add_hand(hnd);
  }

  /* Receive flop from table */
  pub fn add_cards(&mut self, cards: [Card; 3]) {
    self.calc.add_cards(cards);
    self.hand.update(self.calc.calc_hand());
  }

  /* Receive turn/river from table */
  pub fn add(&mut self, card: Card) {
    self.calc.add(card);
    self.hand.update(self.calc.calc_hand());
  }

  /* Display function */
  pub fn display(self) {
    print!("Computer's ");
    self.hand.display();
    println!("Computer's hand value: {}", self.hand.get_combo());
  }
}
