// Copyright © 2018 CS410P Robert Horrace
/*
  This is the Rust file
  containing the deck and
  its functions.
*/

extern crate rand;

use self::rand::{Rng,StdRng};
use game::card::*;

/* Deck struct */
#[derive(Clone,Copy)]
pub struct Deck {
  cards: [Card; 52],  // Array of cards
  head: usize,        // Top of deck (should start at 0)
  tail: usize,        // Bottom of deck (should be 52)
}

/* Functions for Deck */
impl Deck {
  /* The new function */
  pub fn new() -> Deck {
    Deck {
      cards: [Card::new(); 52],
      head: 0,
      tail: 0,
    }
  }
  
  /* Checks the size of the deck */
  pub fn size(self) -> usize {
    self.tail - self.head
  }
  
  /*Build the deck, no Jokers*/
  pub fn build(&mut self) {
    /* Add 52 cards to the deck */
    self.head = 0;
    self.tail = 0;
    for i in 2..15 {
      self.cards[self.tail].change(i,1);
      self.tail += 1;
      self.cards[self.tail].change(i,2);
      self.tail += 1;
      self.cards[self.tail].change(i,3);
      self.tail += 1;
      self.cards[self.tail].change(i,4);
      self.tail += 1;
    }
  }
  
  /* Reset the deck by set head to 0 */
  pub fn reset(&mut self) {
    self.head = 0;
  }

  /* Shuffle the deck */
  pub fn shuffle(&mut self) {
    /* shuffles the deck ten times */
    let mut rng = StdRng::new().unwrap();
    for _ in 0..20 {
      rng.shuffle(&mut self.cards);
    }
  }
  
  /* "Remove" top of the deck, 
      meaning the cards will not actually 
      be removed */
  pub fn rem(&mut self) -> Card {
    self.head += 1;
    self.cards[self.head - 1]
  }
}
